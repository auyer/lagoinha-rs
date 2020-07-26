use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Source {
    Viacep,
    Correios,
    Cepla,
    LagoinhaLib,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::Viacep => write!(f, "Viacep"),
            Source::Correios => write!(f, "Cepla"),
            Source::Cepla => write!(f, "Cepla"),
            Source::LagoinhaLib => write!(f, "Lagoinha"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub source: Source,
    pub kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    /// UnknownServerError represents unmapped server errors
    UnknownServerError {
        code: u16,
    },
    ServerError {
        code: u16,
    },
    ClientError {
        code: u16,
    },
    BodyParsingError {
        error: String,
        body: String,
    },
    AllServicesRetunedErrors {
        e1: String,
        e2: String,
        e3: String,
    },
    MissingBodyError,
    InputError,
    UnexpectedLibraryError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            Kind::UnknownServerError { code } => {
                write!(f, "recieved an unknown error from server with code {} from service {}." , code, self.source)
            },
            Kind::ServerError { code } => {
                write!(f, "Recieved a server error {} from service {}.", code, self.source)
            },
            Kind::ClientError { code } => {
                write!(f, "Recieved a client error {} from service {}.", code, self.source)
            },
            Kind::BodyParsingError { error, body} => {
                write!(f, "Failed to parse body with error {} from service {}. This should not happen, submit this body in a GitHub issue: {}", error, self.source, body)
            },
            Kind::MissingBodyError => {
                write!(f, "Recieved a result without a body from service {}.", self.source)
            },
            Kind::InputError => {
                write!(f, "The CEP is malformatted. It should be follow this templates: 12345-678 or 12345678")
            },
            Kind::UnexpectedLibraryError => {
                write!(f,"Recieved an unexpected error from the library from service {}. Please send an issue in GitHub.", self.source)
            },
            Kind::AllServicesRetunedErrors{e1,e2, e3} => {
                write!(f, "All services returned an error. \n: {}, \n: {}, \n: {}", e1, e2, e3)
            }
        }
    }
}

impl StdError for Error {}
