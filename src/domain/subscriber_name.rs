use unicode_segmentation::UnicodeSegmentation;

/// Represents a valid subscriber name.
///
/// Invariants:
///  - Name is non empty
///  - Name is < 256 graphemes
///  - Name has no forbidden characters: '/()"<>\{}
#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    /// Returns a new SubsriberName if the input satisfies all of the preconditions:
    ///  - Non empty: s.trim().len() > 0
    ///  - Not too long s.graphemes(true).count() < 256
    ///  - no forbidden characters (see list in struct comment)
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        if s.trim().is_empty() {
            return Err(format!("{} is empty, invalid name", s));
        }

        // Consider the number of graphemes in the name, these are user percieved characters.
        // if we have too many input is invalud
        if s.graphemes(true).count() > 256 {
            return Err(format!(
                "{} is too long must be < 256 graphemes, invalid name",
                s
            ));
        }

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        if s.chars().any(|g| forbidden_characters.contains(&g)) {
            return Err(format!("{} contains forbidden characters ", s));
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_grapheme_is_rejected() {
        let name = "ё".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_name_rejected() {
        let name = "    ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_name_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn name_with_forbidden_characters_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Stanley and third ate a bird".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
