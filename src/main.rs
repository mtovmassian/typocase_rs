use std::env;
use std::process;

use typocase::Config;
use typocase::TypoCase;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        usage();
        eprintln!("{}", err);
        process::exit(1);
    });

    let tc = TypoCase::new(config.string);

    match config.transformation.as_str() {
        "pascal"=> println!("{}", tc.pascal_case()),
        "camel"=> println!("{}", tc.camel_case()),
        "snake"=> println!("{}", tc.snake_case()),
        "constant"=> println!("{}", tc.constant_case()),
        "kebab"=> println!("{}", tc.kebab_case()),
        _ => usage()
    };

}

fn usage() {
    let text = "USAGE: 
    typocase [TYPOCASE_TRANSFORMATION] [STRING_TO_TRANSFORM]\n

TYPOCASE_TRANSFORMATION:
    pascal      Pascal case     (ex.: AbcDefGhi)
    camel       Camel case      (ex.: abcDefGhi)
    snake       Snake case      (ex.: abc_def_ghi)
    constant    Constant case   (ex.: ABC_DEF_GHI)
    kebab       Kebab case      (ex.: abc-def-ghi)
";
    println!("{}", text);
}