use async_stream::stream;
use futures::pin_mut;
use std::time::Duration;
use tokio::time;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let s = stream! {
        for i in 0..10 {
            yield i;
            time::sleep(Duration::from_secs(1)).await;
        }
    };

    pin_mut!(s);

    while let Some(value) = s.next().await {
        println!("{value}");
    }
}
