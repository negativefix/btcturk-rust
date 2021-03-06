pub type BTCTRResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    Config(String),
    Network(reqwest::Error),
    JsonParse(serde_json::Error),
    UrlParse(url::ParseError),
    QueryParamsParse(serde_url_params::Error),
}


impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Network(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::JsonParse(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Error {
        Error::UrlParse(e)
    }
}

impl From<serde_url_params::Error> for Error {
    fn from(e: serde_url_params::Error) -> Error {
        Error::QueryParamsParse(e)
    }
}
