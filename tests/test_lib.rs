extern crate typocase;

#[cfg(test)]
mod test_pascal_case {
    use typocase::TypoCase;

    fn fixture(_string: String) -> String {
        let instance = TypoCase::new(_string);

        return instance.pascal_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(String::from("abc def ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(String::from("  aBc_/_DeF,gHI;")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(String::from("AbcDefGhi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(String::from("abcDefGhi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(String::from("abc_def_ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(String::from("ABC_DEF_GHI")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(String::from("abc-def-ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(String::from("abc/def/ghi")),
            String::from("AbcDefGhi")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(String::from("abc.def.ghi")),
            String::from("AbcDefGhi")
        );
    }

}

#[cfg(test)]
mod test_camel_case {
    use typocase::TypoCase;

    fn fixture(_string: String) -> String {
        let instance = TypoCase::new(_string);

        return instance.camel_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(String::from("abc def ghi")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(String::from("  aBc_/_DeF,gHI;")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(String::from("AbcDefGhi")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(String::from("abcDefGhi")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(String::from("abc_def_ghi")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(String::from("ABC_DEF_GHI")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(String::from("abc-def-ghi")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(String::from("abc/def/ghi")),
            String::from("abcDefGhi")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(String::from("abc.def.ghi")),
            String::from("abcDefGhi")
        );
    }

}

#[cfg(test)]
mod test_snake_case {
    use typocase::TypoCase;

    fn fixture(_string: String) -> String {
        let instance = TypoCase::new(_string);

        return instance.snake_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(String::from("abc def ghi")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(String::from("  aBc_/_DeF,gHI;")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(String::from("AbcDefGhi")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(String::from("abcDefGhi")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(String::from("abc_def_ghi")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(String::from("ABC_DEF_GHI")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(String::from("abc-def-ghi")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(String::from("abc/def/ghi")),
            String::from("abc_def_ghi")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(String::from("abc.def.ghi")),
            String::from("abc_def_ghi")
        );
    }

}

#[cfg(test)]
mod test_constant_case {
    use typocase::TypoCase;

    fn fixture(_string: String) -> String {
        let instance = TypoCase::new(_string);

        return instance.constant_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(String::from("abc def ghi")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(String::from("  aBc_/_DeF,gHI;")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(String::from("AbcDefGhi")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(String::from("abcDefGhi")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(String::from("abc_def_ghi")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(String::from("ABC_DEF_GHI")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(String::from("abc-def-ghi")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(String::from("abc/def/ghi")),
            String::from("ABC_DEF_GHI")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(String::from("abc.def.ghi")),
            String::from("ABC_DEF_GHI")
        );
    }

}

#[cfg(test)]
mod test_kebab_case {
    use typocase::TypoCase;

    fn fixture(_string: String) -> String {
        let instance = TypoCase::new(_string);

        return instance.kebab_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(String::from("abc def ghi")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(String::from("  aBc_/_DeF,gHI;")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(String::from("AbcDefGhi")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(String::from("abcDefGhi")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(String::from("abc_def_ghi")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(String::from("ABC_DEF_GHI")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(String::from("abc-def-ghi")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(String::from("abc/def/ghi")),
            String::from("abc-def-ghi")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(String::from("abc.def.ghi")),
            String::from("abc-def-ghi")
        );
    }

}

mod test_path_case {
    use typocase::TypoCase;

    fn fixture(_string: String) -> String {
        let instance = TypoCase::new(_string);

        return instance.path_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(String::from("abc def ghi")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(String::from("  aBc_/_DeF,gHI;")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(String::from("AbcDefGhi")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(String::from("abcDefGhi")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(String::from("abc_def_ghi")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(String::from("ABC_DEF_GHI")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(String::from("abc-def-ghi")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(String::from("abc/def/ghi")),
            String::from("abc/def/ghi")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(String::from("abc.def.ghi")),
            String::from("abc/def/ghi")
        );
    }

}

mod test_dot_case {
    use typocase::TypoCase;

    fn fixture(_string: String) -> String {
        let instance = TypoCase::new(_string);

        return instance.dot_case();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(String::from("")),
            String::from("")
        );
    }

    #[test]
    fn case_normal_string() {
        assert_eq!(
            fixture(String::from("abc def ghi")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_erratic_spec_chars_sep_string() {
        assert_eq!(
            fixture(String::from("  aBc_/_DeF,gHI;")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_pascal_case_string() {
        assert_eq!(
            fixture(String::from("AbcDefGhi")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_camel_case_string() {
        assert_eq!(
            fixture(String::from("abcDefGhi")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_snake_case_string() {
        assert_eq!(
            fixture(String::from("abc_def_ghi")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_constant_case_string() {
        assert_eq!(
            fixture(String::from("ABC_DEF_GHI")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_kebab_case_string() {
        assert_eq!(
            fixture(String::from("abc-def-ghi")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_path_case_string() {
        assert_eq!(
            fixture(String::from("abc/def/ghi")),
            String::from("abc.def.ghi")
        );
    }

    #[test]
    fn case_dot_case_string() {
        assert_eq!(
            fixture(String::from("abc.def.ghi")),
            String::from("abc.def.ghi")
        );
    }

}