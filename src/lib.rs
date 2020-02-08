use std::error::Error;
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

pub struct Influx {
    uri: String,
    stream: Option<TcpStream>,
}

impl Influx {
    pub fn new(uri: String) -> Influx {
        Influx { uri, stream: None }
    }
    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.stream = Some(TcpStream::connect(&self.uri)?);
        Ok(())
    }
    pub fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.stream {
            Some(stream) => stream.shutdown(Shutdown::Both)?,
            None => {}
        }
        Ok(())
    }
}
