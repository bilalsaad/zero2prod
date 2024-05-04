/// Represents a valid subscriber name.
///
/// Invariants:
///  - Name is non empty
///  - Name is < 256 graphemes
///  - Name has no forbidden characters: '/()"<>\{}
#[derive(Debug, Clone)]
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
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

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

    #[test]
    fn valid_emails_are_parsed_succesffully() {
        let email = SafeEmail().fake();
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully2(valid_email: ValidEmailFixture) -> bool {
        dbg!(&valid_email.0);

        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
