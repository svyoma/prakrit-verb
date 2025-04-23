# Prakrit Verb
![GitHub](https://img.shields.io/github/license/svyoma/prakrit-verb)					![GitHub last commit](https://img.shields.io/github/last-commit/svyoma/prakrit-verb)

This repository contains Rust programs for generating Prakrit verb forms.

## Bhūtakāla Form Generator

The `bhUta-kAla` directory contains a Prākṛta Past Tense form generator based on two sūtras of Hemacandrācārya: "sī-hī-hīa bhūtārthasya 8.3.162" and "vyañjanādīyaḥ 8.3.163". It generates forms based on user input, although it may occasionally produce forms that do not exist in Prākṛta due to oversights in the user's input.

### Usage

There is no front-end interface for using this tool. To use it:

-   Clone this repository.
-   Compile `slp_generator.rs` using Rust's `rustc` compiler.
-   Run the compiled executable file.

## Vartamāna Form Generator

The `vartamAna-kAla` directory contains Prākṛta Present Tense form generator based on the rules laid by Hemacandrasūri. It only generates only basic forms. Support for forms with -jja-, -jjA- will be added soon.

Feel free to explore the repository and contribute to its development.

----------
📝 License

This project is licensed under the MIT License. 👤 Author: [Vyom A. Shah](https://github.com/svyoma) – working on tools for classical languages.
