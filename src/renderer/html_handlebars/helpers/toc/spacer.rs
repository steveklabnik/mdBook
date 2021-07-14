// Custom deserializer for `has_sub_items`,
// which allows both strings and plain bool.

use serde::de::{self, Deserializer, Visitor};
use std::fmt;

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
            // notriddle: The old deserializer only checked if the property was present,
            // so we duplicate that behavior here.
            if value == "false" || value == "" {
                warn!(
                    r#"Chapter property `"spacer": "{}"` will cause the spacer to render. This behavior is counterintuitive, and will be changed in a future version."#,
                    value
                );
            }
            Ok(true)
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
