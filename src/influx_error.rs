use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum InfluxError {
    TcpStreamIsNone,
}

impl Error for InfluxError {}

impl fmt::Display for InfluxError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfluxError::TcpStreamIsNone => write!(formatter, "TcpStream is None."),
        }
    }
}
