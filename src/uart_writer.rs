use core::fmt::Write;
use defmt::info;
use embassy_rp::{
    peripherals::UART0,
    uart::{Async, UartTx},
};
use heapless::String;

use crate::transmission_channel;

const UART_BUFFER_SIZE: usize = 16;

#[embassy_executor::task]
pub async fn writer_task(mut tx: UartTx<'static, UART0, Async>) {
    loop {
        let msg = transmission_channel::receive().await;
        info!("sending message: {}", msg);

        let mut buffer: String<UART_BUFFER_SIZE> = String::new();
        let _ = writeln!(buffer, "{msg}");
        let _ = tx.write(buffer.as_bytes()).await;
    }
}
