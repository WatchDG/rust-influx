pub struct MeasurementTag(String, String);
pub struct MeasurementField(String, String);
pub struct Measurement {
    name: String,
    tags: Option<Vec<MeasurementTag>>,
    fields: Vec<MeasurementField>,
    timestamp: Option<i64>,
}