use std::sync::Arc;

use socket_lib::{Command, Response};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::Mutex,
};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let server_address = args.next().unwrap_or_else(|| "127.0.0.1:7890".into());

    let listener = TcpListener::bind(server_address)
        .await
        .expect("can't bind tcp listener");

    let smart_socket = Arc::new(Mutex::new(SmartSocket::default()));

    while let Ok((mut stream, addr)) = listener.accept().await {
        let peer = addr.to_string();
        println!("Peer '{peer}' connected");

        let smart_socket = smart_socket.clone();
        tokio::spawn(async move {
            let mut in_buffer = [0u8];
            while stream.read_exact(&mut in_buffer).await.is_ok() {
                let response = smart_socket
                    .lock()
                    .await
                    .process_command(in_buffer[0].into());
                let response_buf: [u8; 5] = response.into();
                if stream.write_all(&response_buf).await.is_err() {
                    break;
                };
            }

            println!("Connection with {peer} lost. Waiting for new connections...");
        });
    }
}

#[derive(Default)]
struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::TurnOn => {
                self.enabled = true;
                Response::Ok
            }
            Command::TurnOff => {
                self.enabled = false;
                Response::Ok
            }
            Command::IsEnabled => {
                if self.enabled {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::GetPower => {
                if self.enabled {
                    Response::Power(220.5)
                } else {
                    Response::Power(0.0)
                }
            }
            Command::Unknown => {
                println!("Unknown command received");
                Response::Unknown
            }
        }
    }
}
