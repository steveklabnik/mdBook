// Custom deserializer for `has_sub_items`,
// which allows both strings and plain bool.

use serde::de::{self, Deserializer, Visitor};
use std::fmt;
use std::str::FromStr;

pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<bool, D::Error> {
    struct StringOrBool;

    impl<'de> Visitor<'de> for StringOrBool {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("string or bool")
        }

        fn visit_str<E>(self, value: &str) -> Result<bool, E>
        where
            E: de::Error,
        {
            // notriddle: This is identical to what the old deserializer for `has_sub_items` did.
            // It should probably be changed to raise an error on strings other than "false" or "true".
            Ok(FromStr::from_str(value).unwrap_or_default())
        }

        fn visit_bool<E>(self, value: bool) -> Result<bool, E>
        where
            E: de::Error,
        {
            // notriddle: The old deserializer wouldn't allow bool at all, That can be fixed without
            // breaking backwards compatibility, so we do that here.
            Ok(value)
        }
    }

    de.deserialize_any(StringOrBool)
}
