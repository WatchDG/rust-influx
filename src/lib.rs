extern crate string_repr;
extern crate wdg_uri;

pub mod influx_error;

use influx_error::InfluxError;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use string_repr::StringRepr;
use wdg_uri::query::Query;

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
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut n = stream.read(&mut buffer)?;
        let result = buffer[9] == 0x32 && buffer[10] == 0x30 && buffer[11] == 0x34;
        while n == BUFFER_SIZE {
            n = stream.read(&mut buffer)?;
        }
        Ok(result)
    }
    pub fn auth(&mut self, auth: Authentication) -> Result<bool, Box<dyn Error>> {
        self.authentication = auth;
        Ok(true)
    }
    pub fn query_get(&mut self, query: Query) -> Result<String, Box<dyn Error>> {
        let host = self.uri.clone();
        let stream = self.get_stream()?;
        stream.write_fmt(format_args!(
            "GET /query{} HTTP/1.1\r\nHost: {}\r\n\r\n",
            query.string_repr(),
            host
        ))?;
        const BUFFER_SIZE: usize = 2048;
        let mut buffer = [0u8; BUFFER_SIZE];
        let mut n = stream.read(&mut buffer)?;
        let mut string = String::new();
        string.push_str(String::from_utf8_lossy(&buffer).as_ref());
        while n == BUFFER_SIZE {
            n = stream.read(&mut buffer)?;
            string.push_str(String::from_utf8_lossy(&buffer).as_ref());
        }
        Ok(string)
    }
}
