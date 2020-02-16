extern crate wdg_uri;

pub mod influx_error;

use influx_error::InfluxError;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub enum Authentication {
    None,
    BasicAuthentication(String, String),
    QueryParametersAuthentication(String, String),
    RequestBodyAuthentication(String, String),
    JWTAuthentication(String),
}

pub struct Influx {
    uri: String,
    stream: Option<TcpStream>,
    #[allow(dead_code)]
    authentication: Authentication,
}

impl Influx {
    pub fn new(uri: String) -> Influx {
        Influx {
            uri,
            stream: None,
            authentication: Authentication::None,
        }
    }
    fn get_stream(&mut self) -> Result<&mut TcpStream, InfluxError> {
        self.stream
            .as_mut()
            .ok_or_else(|| InfluxError::TcpStreamIsNone)
    }
    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.get_stream()
            .map(|_| ())
            .or_else(|e| -> Result<(), Box<dyn Error>> {
                match e {
                    InfluxError::TcpStreamIsNone => {
                        let stream = TcpStream::connect(&self.uri)?;
                        self.stream = Some(stream);
                        Ok(())
                    }
                }
            })
    }
    pub fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        self.get_stream()?
            .shutdown(Shutdown::Both)
            .or_else(|e| -> Result<(), Box<dyn Error>> { Err(Box::new(e)) })
    }
    pub fn ping(&mut self) -> Result<bool, Box<dyn Error>> {
        let host = self.uri.clone();
        let stream = self.get_stream()?;
        stream.write_fmt(format_args!(
            "HEAD /ping HTTP/1.1\r\nHost: {}\r\n\r\n",
            host
        ))?;
        let mut buffer = [0u8; 12];
        stream.read_exact(&mut buffer)?;
        Ok(buffer[9] == 0x32 && buffer[10] == 0x30 && buffer[11] == 0x34)
    }
}
