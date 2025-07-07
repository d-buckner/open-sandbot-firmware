use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

const TRANSMISSION_CHANNEL_SIZE: usize = 8;

static TRANSMISSION_CHANNEL: Channel<ThreadModeRawMutex, &str, TRANSMISSION_CHANNEL_SIZE> =
    Channel::new();

pub async fn send(msg: &'static str) {
    TRANSMISSION_CHANNEL.send(msg).await
}

pub async fn receive() -> &'static str {
    TRANSMISSION_CHANNEL.receive().await
}
