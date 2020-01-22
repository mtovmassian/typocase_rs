pub mod core;
use crate::core::str_comps::StringCompounds;

pub struct TypoCase {
    compounds: Vec<String>
}

impl TypoCase {
    pub fn new(_string: &String) -> TypoCase {
        return TypoCase { 
            compounds: StringCompounds::new(_string).extract()
        };
    }

    pub fn pascal_case(&self) -> String {
        return self.compounds.iter()
        .map(|s| {
            let (head, tail) = (&s[0..1], &s[1..]);
            return format!("{}{}", head.to_uppercase(), tail)
        })
        .collect::<Vec<String>>().join("");
    }

}