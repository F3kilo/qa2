use std::{net::SocketAddr, time::Duration};

use tokio::{
    net::UdpSocket,
    time::{self, Instant},
};

#[tokio::main]
async fn main() {
    let args = std::env::args();
    let mut args = args.skip(1);

    let receiver = args.next().unwrap_or_else(|| "127.0.0.1:4321".into());

    println!("Receiver address from args: {receiver}");

    let receiver = receiver
        .parse::<SocketAddr>()
        .expect("valid socket address expected");

    let bind_addr = "127.0.0.1:4320";
    let socket = UdpSocket::bind(bind_addr).await.expect("can't bind socket");
    let temperature_generator = TemperatureGenerator::default();

    println!("Starting send temperature from {bind_addr} to {receiver}");
    loop {
        let temperature = temperature_generator.generate();
        let bytes = temperature.to_be_bytes();
        let send_result = socket.send_to(&bytes, receiver).await;
        if let Err(err) = send_result {
            println!("can't send temperature: {err}")
        }

        let duration = Duration::from_secs(1);
        time::sleep(duration).await;
    }
}

struct TemperatureGenerator {
    started: Instant,
}

impl Default for TemperatureGenerator {
    fn default() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

impl TemperatureGenerator {
    pub fn generate(&self) -> f32 {
        let delay = Instant::now() - self.started;
        20.0 + (delay.as_secs_f32() / 2.0).sin()
    }
}
