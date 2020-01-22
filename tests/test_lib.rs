extern crate typocase_rs;

#[cfg(test)]
mod test_pascal_case {
    use typocase_rs::TypoCase;

    fn fixture(_string: &String) -> String {
        let instance = TypoCase::new(_string);

        return instance.pascal_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(&String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(&String::from("abc def ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(&String::from("  aBc_/_DeF,gHI;")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(&String::from("AbcDefGhi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(&String::from("abcDefGhi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(&String::from("abc_def_ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(&String::from("ABC_DEF_GHI")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(&String::from("abc-def-ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(&String::from("abc/def/ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(&String::from("abc.def.ghi")),
            String::from("AbcDefGhi")
        );
    }

}