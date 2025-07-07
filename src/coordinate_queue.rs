use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};

use crate::coordinate::PolarCoordinate;

const COORDINATE_QUEUE_SIZE: usize = 512;

static COORDINATE_CHANNEL: Channel<ThreadModeRawMutex, PolarCoordinate, COORDINATE_QUEUE_SIZE> =
    Channel::new();

pub async fn queue(coordinate: PolarCoordinate) {
    let _ = COORDINATE_CHANNEL.send(coordinate).await;
}

pub async fn dequeue() -> PolarCoordinate {
    COORDINATE_CHANNEL.receive().await
}
