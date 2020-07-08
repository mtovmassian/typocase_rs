# Typocase RS

![](static/printer.png)

___

CLI for translating string into typography conventions.

## How it works

- Accept any string as input.
- Detect substrings based on special chars or uppercase letters as separators.
- Join substrings based on the typography convention selected:
  - snake case
  - pascal case
  - camel case
  - constant case
  - kebab case

## Usage
```bash
$ typocase pascal abc_def_ghi
AbcDefGhi
```

## License
This software is released under the MIT LICENSE.

## Author
Martin Tovmassian
