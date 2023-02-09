#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Value not of type '{0}'")]
    ValueNotOfType(&'static str),

    #[error("Property not found '{0}'")]
    PropertyNotFound(String),
    
    #[error("Failed to create - {0}")]
    DatabaseFailedToCreate(String),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
