use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::Mutex,
    time,
};

pub struct SmartThermo {
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
}

impl SmartThermo {
    pub async fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address).await?;
        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Temperature::default());

        let temperature_clone = temperature.clone();
        let finished_clone = finished.clone();

        let timeout = Duration::from_secs(1);

        tokio::spawn(async move {
            loop {
                if finished_clone.load(Ordering::SeqCst) {
                    return;
                }

                let mut buf = [0; 4];
                if let Err(e) = time::timeout(timeout, socket.recv_from(&mut buf)).await {
                    println!("can't receive datagram: {e}");
                    continue;
                }

                let val = f32::from_be_bytes(buf);
                temperature_clone.set(val).await;
            }
        });

        Ok(Self {
            temperature,
            finished,
        })
    }

    pub async fn get_temperature(&self) -> f32 {
        self.temperature.get().await
    }
}

impl Drop for SmartThermo {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst)
    }
}

#[derive(Default)]
struct Temperature(Mutex<f32>);

impl Temperature {
    pub async fn get(&self) -> f32 {
        *self.0.lock().await
    }

    pub async fn set(&self, val: f32) {
        *self.0.lock().await = val
    }
}
