/// Represents a valid subscriber name.
///
/// Invariants:
///  - Name is non empty
///  - Name is < 256 graphemes
///  - Name has no forbidden characters: '/()"<>\{}
#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    /// Returns a new SubscriberEmail if the input satisfies all of the preconditions:
    ///  - passed validator crates validate_email
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validator::validate_email(&s) {
            return Ok(Self(s));
        }
        return Err(format!("{} invalid email", s));
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use claims::{assert_err, assert_ok};

    #[test]
    fn valid_email_accepted() {
        let email = "cat@cat.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}
