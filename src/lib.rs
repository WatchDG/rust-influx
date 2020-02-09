use std::error::Error;
use std::fmt;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[allow(dead_code)]
struct MeasurementTag {
    key: String,
    value: String,
}

#[allow(dead_code)]
struct MeasurementField {
    key: String,
    value: String,
}

#[allow(dead_code)]
struct Measurement {
    name: String,
    tags: Option<Vec<MeasurementTag>>,
    fields: Vec<MeasurementField>,
    timestamp: Option<i64>,
}

#[derive(Debug)]
struct InfluxError(String);

impl fmt::Display for InfluxError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Error: {}", self.0)
    }
}

impl Error for InfluxError {}

pub struct Influx {
    uri: String,
    stream: Option<TcpStream>,
}

impl Influx {
    pub fn new(uri: String) -> Influx {
        Influx { uri, stream: None }
    }
    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        let stream = TcpStream::connect(&self.uri)?;
        stream.set_nodelay(true)?;
        self.stream = Some(stream);
        Ok(())
    }
    pub fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.stream {
            Some(stream) => stream.shutdown(Shutdown::Both)?,
            None => {}
        }
        Ok(())
    }
    pub fn ping(&mut self) -> Result<bool, Box<dyn Error>> {
        match &mut self.stream {
            Some(stream) => {
                stream.write(
                    format!("HEAD /ping HTTP/1.1\r\nHost: {}\r\n\r\n", self.uri).as_bytes(),
                )?;
                let mut buffer = [0u8; 12];
                stream.read(&mut buffer)?;
                Ok(buffer[9] == 0x32 && buffer[10] == 0x30 && buffer[11] == 0x34)
            }
            None => Err(Box::new(InfluxError("no stream".into()))),
        }
    }
}
