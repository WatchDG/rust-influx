#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct MeasurementTag {
    key: String,
    value: String,
}

struct MeasurementField {
    key: String,
    value: String,
}

struct Measurement {
    name: String,
    tags: Option<Vec<MeasurementTag>>,
    fields: Vec<MeasurementField>,
    timestamp: Option<i64>,
}
