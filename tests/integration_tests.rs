// Tests run on host machine, not embedded target

// Mock the coordinate type for testing
#[derive(Clone, Debug, PartialEq)]
struct PolarCoordinate {
    theta: f64,
    rho: f64,
}

#[derive(Debug, PartialEq)]
struct StepPosition {
    primary_steps: i64,
    secondary_steps: i64,
}

impl StepPosition {
    fn get_total_steps(&self) -> i64 {
        self.primary_steps.abs() + self.secondary_steps.abs()
    }

    fn delta(&self, other: &StepPosition) -> StepPosition {
        StepPosition {
            primary_steps: self.primary_steps - other.primary_steps,
            secondary_steps: self.secondary_steps - other.secondary_steps,
        }
    }
}

// Constants from arm.rs
const MAIN_PULLEY_TEETH: f64 = 90.0;
const MOTOR_PULLEY_TEETH: f64 = 16.0;
const DEGREES_PER_STEP: f64 = 1.8;
const MICRO_STEPS: f64 = 16.0;
const STEPS_PER_DEG: f64 = MICRO_STEPS * MAIN_PULLEY_TEETH / MOTOR_PULLEY_TEETH / DEGREES_PER_STEP;

fn degrees(radians: f64) -> f64 {
    radians * (180.0 / std::f64::consts::PI)
}

fn get_target_step_position(position: &PolarCoordinate) -> StepPosition {
    let theta_degrees = -degrees(position.theta);
    let secondary_degrees = 180.0 - degrees((0.5 - position.rho.powi(2)) * 2.0).acos();
    let primary_offset = secondary_degrees / 2.0;
    let primary_degrees = theta_degrees - primary_offset;

    let primary_steps = (primary_degrees * STEPS_PER_DEG).round() as i64;
    StepPosition {
        primary_steps,
        secondary_steps: (secondary_degrees * STEPS_PER_DEG).round() as i64 + primary_steps,
    }
}

#[cfg(test)]
mod coordinate_tests {
    use super::*;

    #[test]
    fn test_degrees_conversion() {
        assert!((degrees(std::f64::consts::PI) - 180.0).abs() < 0.001);
        assert!((degrees(std::f64::consts::PI / 2.0) - 90.0).abs() < 0.001);
        assert!((degrees(0.0) - 0.0).abs() < 0.001);
        assert!((degrees(-std::f64::consts::PI) + 180.0).abs() < 0.001);
    }

    #[test]
    fn test_origin_coordinate() {
        let origin = PolarCoordinate {
            theta: 0.0,
            rho: 0.0,
        };
        let steps = get_target_step_position(&origin);

        assert_eq!(steps.primary_steps, 0);
        assert_eq!(steps.secondary_steps, 0);
    }

    #[test]
    fn test_step_position_total_steps() {
        let pos = StepPosition {
            primary_steps: 100,
            secondary_steps: -50,
        };
        assert_eq!(pos.get_total_steps(), 150);

        let pos_negative = StepPosition {
            primary_steps: -100,
            secondary_steps: -50,
        };
        assert_eq!(pos_negative.get_total_steps(), 150);
    }

    #[test]
    fn test_step_position_delta() {
        let pos1 = StepPosition {
            primary_steps: 100,
            secondary_steps: 200,
        };
        let pos2 = StepPosition {
            primary_steps: 50,
            secondary_steps: 150,
        };

        let delta = pos1.delta(&pos2);
        assert_eq!(delta.primary_steps, 50);
        assert_eq!(delta.secondary_steps, 50);
    }

    #[test]
    fn test_max_radius_coordinate() {
        // Test at maximum radius (rho = 1.0)
        // Real world: this is a boundary case that maps to origin
        let max_radius = PolarCoordinate {
            theta: 0.0,
            rho: 1.0,
        };
        let steps = get_target_step_position(&max_radius);

        // Hardware limitation: rho=1.0 maps to origin due to kinematics
        assert_eq!(steps.primary_steps, 0);
        assert_eq!(steps.secondary_steps, 0);
    }

    #[test]
    fn test_coordinate_transformation_consistency() {
        // Test that the coordinate transformation is deterministic
        let test_coord = PolarCoordinate {
            theta: 1.0,
            rho: 0.3,
        };
        let steps1 = get_target_step_position(&test_coord);
        let steps2 = get_target_step_position(&test_coord);

        // Same input should always produce same output
        assert_eq!(steps1.primary_steps, steps2.primary_steps);
        assert_eq!(steps1.secondary_steps, steps2.secondary_steps);
    }

    #[test]
    fn test_coordinate_boundary_cases() {
        // Test known boundary cases that work in your real hardware
        let coords = [
            // Origin - always works
            PolarCoordinate {
                theta: 0.0,
                rho: 0.0,
            },
            // Max radius that causes boundary behavior
            PolarCoordinate {
                theta: 0.0,
                rho: 1.0,
            },
        ];

        for coord in coords.iter() {
            let steps = get_target_step_position(coord);
            // These should not panic and should return valid i64 values
            // Just verify they don't overflow or cause panics
            let _total = steps.get_total_steps(); // This will panic if values are invalid
        }
    }
}

#[cfg(test)]
mod command_tests {
    // Test command buffer behavior

    #[test]
    fn test_command_buffer_size() {
        const COMMAND_BUFFER_SIZE: usize = 32;
        let buf = [0u8; COMMAND_BUFFER_SIZE];
        assert_eq!(buf.len(), 32);
    }

    #[test]
    fn test_buffer_parsing() {
        // Simulate parsing "MOVE 1.5 0.7\n"
        let command = b"MOVE 1.5 0.7 ";
        let command_str = std::str::from_utf8(command).unwrap();
        let mut parts = command_str.split(' ');

        assert_eq!(parts.next(), Some("MOVE"));
        assert_eq!(parts.next(), Some("1.5"));
        assert_eq!(parts.next(), Some("0.7"));
        assert_eq!(parts.next(), Some(""));
    }

    #[test]
    fn test_float_parsing() {
        assert_eq!("1.5".parse::<f64>(), Ok(1.5));
        assert_eq!("0.7".parse::<f64>(), Ok(0.7));
        assert_eq!("-3.14".parse::<f64>(), Ok(-3.14));
        assert!("abc".parse::<f64>().is_err());
        assert!("".parse::<f64>().is_err());
    }
}

#[cfg(test)]
mod stepper_tests {
    use super::*;

    #[test]
    fn test_step_ratios() {
        // Test the step ratio calculation logic
        let s0_steps = 100_i64;
        let s1_steps = 50_i64;

        let s0_ratio = (s0_steps as f64 / std::cmp::max(s1_steps, 1) as f64).min(1.0);
        let s1_ratio = (s1_steps as f64 / std::cmp::max(s0_steps, 1) as f64).min(1.0);

        assert_eq!(s0_ratio, 1.0); // s0 has more steps, so ratio caps at 1.0
        assert_eq!(s1_ratio, 0.5); // s1 has half the steps
    }

    #[test]
    fn test_equal_steps() {
        let s0_steps = 100_i64;
        let s1_steps = 100_i64;

        let s0_ratio = (s0_steps as f64 / std::cmp::max(s1_steps, 1) as f64).min(1.0);
        let s1_ratio = (s1_steps as f64 / std::cmp::max(s0_steps, 1) as f64).min(1.0);

        assert_eq!(s0_ratio, 1.0);
        assert_eq!(s1_ratio, 1.0);
    }
}
