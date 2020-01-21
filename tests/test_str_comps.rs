extern crate typocase_rs;

#[cfg(test)]
mod test_split_on_upper_case_letters {
    use typocase_rs::str_comps;

    fn fixture(_string: &String) -> Vec<String> {
        let instance = str_comps::StringCompounds::from(_string);

        return instance.split_on_uppercase_letters();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(&String::from("")),
            Vec::new() as Vec<String>
        );
    }

    #[test]
    fn case_uppercase_letter_sep() {
        assert_eq!(
            fixture(&String::from("AbcDefGhi")),
            vec!["abc", "def", "ghi"]
        );
        assert_eq!(
            fixture(&String::from("abcDefGhi")),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn case_uppercase_letter_sep_and_leading_trailing_special_char() {
        assert_eq!(
            fixture(&String::from("_AbcDefGhi_")),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn case_only_uppercase_letter_sep() {
        assert_eq!(
            fixture(&String::from("ABCDEFGHI")),
            vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"]
        );
    }

    #[test]
    fn case_uppercase_letter_sep_and_special_char_sep() {
        assert_eq!(
            fixture(&String::from("A_b_c_D_e_f_G_h_i")),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn case_no_uppercase_letter_sep() {
        assert_eq!(
            fixture(&String::from("abcdefghi")),
            vec!["abcdefghi"]
        );
    }

    #[test]
    fn case_no_uppercase_letter_sep_and_special_char_sep() {
        assert_eq!(
            fixture(&String::from("abc_def_ghi")),
            vec!["abcdefghi"]
        );
    }

}

#[cfg(test)]
mod test_split_on_special_char {
    use typocase_rs::str_comps;

    fn fixture(_string: &String) -> Vec<String> {
        let instance = str_comps::StringCompounds::from(_string);

        return instance.split_on_special_char();
    }

    #[test]
    fn case_empty_string() {
        assert_eq!(
            fixture(&String::from("")),
            Vec::new() as Vec<String>
        );
    }

    #[test]
    fn case_special_char_sep() {
        assert_eq!(
            fixture(&String::from("abc_def_ghi")),
            vec!["abc", "def", "ghi"]
        );
        assert_eq!(
            fixture(&String::from("abc/def/ghi")),
            vec!["abc", "def", "ghi"]
        );
        assert_eq!(
            fixture(&String::from("abc@def.ghi")),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn case_multi_special_char_sep() {
        assert_eq!(
            fixture(&String::from("abc_|_def__@__ghi")),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn case_special_char_sep_and_leading_trailing_special_char() {
        assert_eq!(
            fixture(&String::from("_abc_def_ghi_")),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn case_special_char_sep_and_uppercase_letters() {
        assert_eq!(
            fixture(&String::from("ABC_DEF_GHI")),
            vec!["abc", "def", "ghi"]
        );
    }

    #[test]
    fn no_special_char_sep() {
        assert_eq!(
            fixture(&String::from("abcdefghi")),
            vec!["abcdefghi"]
        );
    }

    #[test]
    fn no_special_char_sep_and_uppercase_letters() {
        assert_eq!(
            fixture(&String::from("abcDefGhi")),
            vec!["abcdefghi"]
        );
    }

}