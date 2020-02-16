pub struct Tag(String, String);
pub struct Field(String, String);
pub struct Measurement {
    name: String,
    tags: Option<Vec<Tag>>,
    fields: Vec<Field>,
    timestamp: Option<i64>,
}