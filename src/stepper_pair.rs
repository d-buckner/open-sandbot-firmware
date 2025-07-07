use crate::stepper::Stepper;
use embassy_rp::gpio::{AnyPin, Level, Output};
use embassy_rp::PeripheralRef;
use embassy_time::Timer;

const DEFAULT_SPEED: u8 = 90;
const SPEED_SCALE_FACTOR: u64 = 100;
const BASE_DELAY_MICROS: u64 = 250;

pub struct StepperPair<'a> {
    stepper_0: Stepper<'a>,
    stepper_1: Stepper<'a>,
    enable_output: Output<'a, AnyPin>,
    speed: u8,
}

pub struct StepperPairPins {
    pub stepper0_step_pin: PeripheralRef<'static, AnyPin>,
    pub stepper0_dir_pin: PeripheralRef<'static, AnyPin>,
    pub stepper1_step_pin: PeripheralRef<'static, AnyPin>,
    pub stepper1_dir_pin: PeripheralRef<'static, AnyPin>,
    pub stepper_enable_pin: PeripheralRef<'static, AnyPin>,
}

impl StepperPair<'_> {
    pub fn new(pins: StepperPairPins) -> Self {
        let mut stepper_pair = StepperPair {
            stepper_0: Stepper::new(pins.stepper0_step_pin, pins.stepper0_dir_pin),
            stepper_1: Stepper::new(pins.stepper1_step_pin, pins.stepper1_dir_pin),
            enable_output: Output::new(pins.stepper_enable_pin, Level::Low),
            speed: DEFAULT_SPEED,
        };
        stepper_pair.disengage();
        stepper_pair
    }

    pub async fn move_to(&mut self, stepper_0_steps: i64, stepper_1_steps: i64) {
        self.engage();
        let is_s0_forward = stepper_0_steps >= 0;
        let is_s1_forward = stepper_1_steps < 0;
        let s0_steps_abs = stepper_0_steps.abs();
        let s1_steps_abs = stepper_1_steps.abs();
        let s0_ratio = (s0_steps_abs as f64 / (s1_steps_abs as f64).max(1.0)).min(1.0);
        let s1_ratio = (s1_steps_abs as f64 / (s0_steps_abs as f64).max(1.0)).min(1.0);

        let mut s0_partial_steps = 0.0;
        let mut s1_partial_steps = 0.0;
        let mut s0_steps = 0;
        let mut s1_steps = 0;

        while s0_steps_abs - s0_steps > 0 || s1_steps_abs - s1_steps > 0 {
            s0_partial_steps += s0_ratio;
            s1_partial_steps += s1_ratio;

            if s0_partial_steps >= 1.0 {
                self.stepper_0.step(is_s0_forward);
                s0_steps += 1;
                s0_partial_steps -= 1.0;
            }

            if s1_partial_steps >= 1.0 {
                self.stepper_1.step(is_s1_forward);
                s1_steps += 1;
                s1_partial_steps -= 1.0;
            }
            let delay =
                ((SPEED_SCALE_FACTOR - self.speed as u64) * SPEED_SCALE_FACTOR) + BASE_DELAY_MICROS;
            Timer::after_micros(delay).await;
        }
    }

    fn engage(&mut self) {
        self.enable_output.set_low();
    }

    fn disengage(&mut self) {
        self.enable_output.set_high();
    }
}
