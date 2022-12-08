pub mod error_handler {
    use thiserror::Error;
    
    #[derive(Debug, Error)]
    pub enum FileError {
        #[error("Database URL could not be found! Please make sure that you run 'set-database' and pass the URL of your Postgres database.")]
        FileMissing(#[from] std::io::Error),

        #[error("Error occurred when reading your database URL. Please run 'set-database' and pass the URL of your Postgres database.")]
        FileNotReadable(#[from] serde_yaml::Error),
    }

    #[derive(Debug, Error)]
    pub enum YahooError {
        #[error("Could not get value for {value:?}")]
        ParseError {
            value: String
        },

        #[error("Could not parse data for {value:?}")]
        RegexError {
            value: String,
        },
    }
}