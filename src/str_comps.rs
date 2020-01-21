const WHITE_SPACE: char = ' ';

pub struct StringCompounds {
    _string: String
}

impl StringCompounds {

    pub fn from(_string: &String) -> StringCompounds {
        return StringCompounds { _string: _string.clone() };
    }

    pub fn split_on_uppercase_letters(&self) -> Vec<String> {
        let alphanums: String = self._string.chars()
        .filter(|_char|_char.is_alphanumeric())
        .map(|_char| {
            if _char.is_lowercase() {
                return _char.to_string();
            } else {
                return format!("{}{}", WHITE_SPACE, _char.to_lowercase());
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
                return WHITE_SPACE.to_string();
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
