use crate::error::AppError;

pub trait FileMapHelper: Default {
    type Return;

    fn new() -> Self {
        Self::default()
    }

    fn generate_hash(&self, text: &str) -> Result<String, AppError>;
    fn line_to_object(&self, text: &str) -> Option<Self::Return>;
}
