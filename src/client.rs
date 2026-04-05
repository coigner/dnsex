use crate::error::DnsexError;

pub struct Client {
    domain: String,
    file: Option<String>,
}

impl Client {
    pub fn new(domain: impl Into<String>, file: Option<String>) -> Self {
        Self {
            domain: domain.into(),
            file,
        }
    }

    pub fn query() -> Result<(), DnsexError> {
        Ok(())
    }
}
