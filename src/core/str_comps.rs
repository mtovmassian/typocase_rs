use crate::core::spec_chars::SpecChars;

pub struct StringCompounds {
    _string: String
}

impl StringCompounds {

    pub fn new(_string: &String) -> StringCompounds {
        return StringCompounds { _string: _string.clone() };
    }

    pub fn extract(&self) -> Vec<String> {
        let mut compounds = self.split_on_special_char();
        if compounds.len() == 1 {
            compounds = self.split_on_uppercase_letters();
        }

        return compounds;
    }

    pub fn split_on_uppercase_letters(&self) -> Vec<String> {
        let alphanums: String = self._string.chars()
        .filter(|_char|_char.is_alphanumeric())
        .map(|_char| {
            if _char.is_lowercase() {
                return _char.to_string();
            } else {
                return format!(
                    "{}{}", SpecChars::WhiteSpace.to_string(), _char.to_lowercase()
                );
            }
        })
        .collect::<String>();

        let compounds = self.split_on_whitespace(alphanums);

        return compounds;
    }

    pub fn split_on_special_char(&self) -> Vec<String> {
        let alphanums: String = self._string.chars()
        .map(|_char| {
            if _char.is_alphanumeric() {
                return _char.to_string().to_lowercase();
            } else {
                return SpecChars::WhiteSpace.to_string();
            }
        })
        .collect::<String>();

        let compounds = self.split_on_whitespace(alphanums);

        return compounds;
    }

    fn split_on_whitespace(&self, _string: String) -> Vec<String> {
        return _string.split_whitespace()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    }

}
