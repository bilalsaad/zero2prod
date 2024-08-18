/// Strong string type for an IdempotencyKey stored in the db.
/// REQUIRES:
///  - len(s) > 0 and len(s) < 50.
#[derive(Debug)]
pub struct IdempotencyKey(String);

impl TryFrom<String> for IdempotencyKey {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.is_empty() {
            anyhow::bail!("The IdempotencyKey canot be empty.");
        }
        let max_length = 50;
        if s.len() >= max_length {
            anyhow::bail!("The IdempotencyKey must be shorter than {max_length} characters");
        }
        Ok(Self(s))
    }
}

impl From<IdempotencyKey> for String {
    fn from(k: IdempotencyKey) -> Self {
        return k.0;
    }
}

impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}
