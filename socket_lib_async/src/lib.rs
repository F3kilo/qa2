use std::{error::Error, fmt};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
};

pub enum Command {
    TurnOff,
    TurnOn,
    IsEnabled,
    GetPower,
    Unknown,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::TurnOff,
            1 => Self::TurnOn,
            2 => Self::IsEnabled,
            3 => Self::GetPower,
            _ => Self::Unknown,
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::TurnOff => 0,
            Command::TurnOn => 1,
            Command::IsEnabled => 2,
            Command::GetPower => 3,
            Command::Unknown => 255,
        }
    }
}

pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Power(f32),
    Unknown,
}

impl From<[u8; 5]> for Response {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes {
            [0, ..] => Self::Ok,
            [1, ..] => Self::Enabled,
            [2, ..] => Self::Disabled,
            [3, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Power(f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}

impl From<Response> for [u8; 5] {
    fn from(resp: Response) -> Self {
        let mut buffer = [0u8; 5];
        match resp {
            Response::Ok => {}
            Response::Enabled => buffer[0] = 1,
            Response::Disabled => buffer[0] = 2,
            Response::Power(pwr) => {
                buffer[0] = 3;
                buffer[1..].copy_from_slice(&pwr.to_be_bytes())
            }
            Response::Unknown => buffer[0] = 255,
        };
        buffer
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Response::Ok => write!(f, "Ok"),
            Response::Enabled => write!(f, "Enabled"),
            Response::Disabled => write!(f, "Disabled"),
            Response::Power(power) => write!(f, "Power: {}", power),
            Response::Unknown => write!(f, "Unknown"),
        }
    }
}

pub struct SmartSocketClient {
    stream: TcpStream,
}

impl SmartSocketClient {
    pub async fn new(server_address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_address).await?;
        Ok(Self { stream })
    }

    pub async fn run_command(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()]).await?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer).await?;
        Ok(buffer.into())
    }
}
