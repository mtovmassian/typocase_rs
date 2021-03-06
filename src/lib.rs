pub mod core;
use crate::core::str_comps::StringCompounds;
use crate::core::spec_chars::SpecChars;

pub struct TypoCase {
    compounds: Vec<String>
}

impl TypoCase {

    pub fn new(_string: String) -> TypoCase {
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

    pub fn camel_case(&self) -> String {
        let pascal_case = self.pascal_case();
        if pascal_case.len() == 0 {
            return pascal_case;
        }
        let (head, tail) = (&pascal_case[0..1], &pascal_case[1..]);
        
        return format!("{}{}", head.to_lowercase(), tail);
    }

    pub fn snake_case(&self) -> String {
        return self.join_on_spec_char(SpecChars::UnderScore);
    }

    pub fn constant_case(&self) -> String {
        return self.snake_case().to_uppercase();
    }

    pub fn kebab_case(&self) -> String {
        return self.join_on_spec_char(SpecChars::Dash);
    }

    fn join_on_spec_char(&self, spec_char: SpecChars) -> String {
        return self.compounds.iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(&spec_char.to_string());
    }

}

pub struct Config {
    pub transformation: String,
    pub string: String
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let transformation = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing typocase transformation argument."),
        };
        let string = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing string to transform argument."),
        };

        Ok(Config { transformation, string })
    }
}