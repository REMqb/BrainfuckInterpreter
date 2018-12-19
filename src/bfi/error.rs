use std::fmt;

#[derive(Debug)]
pub enum Error {
    LastInstructionReached,
    MissingLeftBracket,
    MissingRightBracket,
    NotReady
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Error::LastInstructionReached => "Last instruction reached",
            Error::MissingLeftBracket => "Missing left bracket",
            Error::MissingRightBracket => "Missing right bracket",
            Error::NotReady => "Not ready (end of program or error encountered)"
        })
    }
}