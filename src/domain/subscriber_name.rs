use tracing::{error, info, instrument, warn};
use unicode_segmentation::UnicodeSegmentation;

const MAX_NAME_LENGTH: usize = 256;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    #[instrument(name = "parse name", level = "error", skip_all)]
    pub fn parse(input: String) -> Result<Self, String> {
        info!("parsing subscriber name");
        if input.trim().is_empty() {
            error!(reason = "no name supplied", "failed to parse name");
            return Err(String::from("empty name field"));
        }
        if input.graphemes(true).count() > MAX_NAME_LENGTH {
            error!(reason = "character limit exceeded", "failed to parse name");
            return Err(format!(
                "name {input} greater than character limit {MAX_NAME_LENGTH}"
            ));
        }
        let invalid_characters = ['/', '\\', '(', ')', '"', '<', '>', '{', '}'];
        if input.chars().any(|c| invalid_characters.contains(&c)) {
            error!(reason = "invalid characters", "failed to parse name");
            return Err(format!("name {input} contains invalid characters"));
        }

        info!("parsed name successfully");
        Ok(Self(input))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
