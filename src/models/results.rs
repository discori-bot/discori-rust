pub struct ModelMutationResult {
    pub id: String,
}

impl From<String> for ModelMutationResult {
    fn from(id: String) -> Self {
        Self { id }
    }
}
