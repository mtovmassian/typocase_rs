const WHITE_SPACE: char = ' ';

pub struct StringCompounds {
    _string: String
}

impl StringCompounds {

    pub fn from(_string: &String) -> StringCompounds {
        return StringCompounds { _string: _string.clone() };
    }

    pub fn split_on_special_char(&self) -> Vec<String> {
        let alphanum_chars = self._string.chars()
        .map(|_char| {
            if _char.is_alphanumeric() {
                return _char;
            } else {
                return WHITE_SPACE;
            }
        })
        .collect::<Vec<char>>();

        let compounds = alphanum_chars.iter().collect::<String>()
        .split_whitespace()
        .map(|s| String::from(s).to_lowercase())
        .collect::<Vec<String>>();
        println!("{:?}", compounds);
        println!("{}", self._string);

        return compounds;
    }

}
