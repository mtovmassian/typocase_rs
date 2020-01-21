#[allow(dead_code)]
pub enum SpecChars {
    Dash,
    Dot,
    Slash,
    UnderScore,
    WhiteSpace,
}


impl SpecChars {
    pub fn value(&self) -> char {
        match self {
            SpecChars::Dash => '-',
            SpecChars::Dot => '.',
            SpecChars::Slash => '/',
            SpecChars::UnderScore => '_',
            SpecChars::WhiteSpace => ' '
        }
    }

    pub fn to_string(&self) -> String {
        return self.value().to_string();
    }
}
