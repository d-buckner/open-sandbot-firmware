use defmt::info;
use embassy_rp::{
    peripherals::UART0,
    uart::{Async, UartRx},
};

use crate::messages::error;
use crate::{
    command::Command, coordinate::PolarCoordinate, coordinate_queue, transmission_channel,
};

static MOVE: &str = "MOVE";

#[embassy_executor::task]
pub async fn reader_task(mut rx: UartRx<'static, UART0, Async>) {
    loop {
        let mut command = Command::new();
        loop {
            let mut char_buf = [0u8];
            let _rr = rx.read(&mut char_buf).await;
            let _ = command.add_char_buf(&char_buf);
            if command.is_complete() {
                break;
            }
        }

        let input = match command.to_str() {
            Ok(s) => s,
            Err(_) => {
                info!("received invalid utf-8");
                transmission_channel::send(error::INVALID_UTF8).await;
                continue;
            }
        };
        info!("received message: {}", input);
        let mut args = input.split(' ');
        let method = match args.next() {
            Some(m) => m,
            None => continue, // empty command
        };

        if method == MOVE {
            // extract theta and rho arguments
            let (theta_str, rho_str) = match (args.next(), args.next()) {
                (Some(t), Some(r)) => (t, r),
                _ => {
                    info!("move command missing arguments");
                    transmission_channel::send(error::MISSING_ARGS).await;
                    continue;
                }
            };

            // parse the numbers
            let (theta, rho) = match (theta_str.parse::<f64>(), rho_str.parse::<f64>()) {
                (Ok(t), Ok(r)) => (t, r),
                _ => {
                    info!("move command has invalid numbers");
                    transmission_channel::send(error::INVALID_NUMBERS).await;
                    continue;
                }
            };

            coordinate_queue::queue(PolarCoordinate { theta, rho }).await;
        }
    }
}
