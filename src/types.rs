pub type DocId = u32;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocId,
    pub path: String,
    pub text: String,
}
