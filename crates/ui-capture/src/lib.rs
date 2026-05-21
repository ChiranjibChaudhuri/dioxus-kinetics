#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaptureStageDescriptor {
    pub id: String,
}

impl CaptureStageDescriptor {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
