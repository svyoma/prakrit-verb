# Prakrit Verb
![GitHub](https://img.shields.io/github/license/svyoma/prakrit-verb)					![GitHub last commit](https://img.shields.io/github/last-commit/svyoma/prakrit-verb)

This repository contains Rust programs for generating Prakrit verb forms.

## Bhūtakāla Form Generator

The `bhuta-kala` directory contains a Prākṛta Past Tense form generator based on two sūtras of Hemacandrācārya: "sī-hī-hīa bhūtārthasya 8.3.162" and "vyañjanādīyaḥ 8.3.163". It generates forms based on user input, although it may occasionally produce forms that do not exist in Prākṛta due to oversights in the user's input.

### Usage

There is no front-end interface for using this tool. To use it:

-   Clone this repository.
-   Compile either `dev_generator.rs` or `slp_generator.rs` using Rust's `rustc` compiler.
-   Run the compiled executable file.

Alternatively, you can copy the code from `dev_generator.rs` or `slp_generator.rs` and run it on the [Rust Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024).

## Vartamāna Form Generator

Karnatak Samskrit University has developed and deployed a present tense verb form generator in Python on their [Sambhāṣa platform](https://sambhasha.ksu.ac.in/CompLing/prakrit_verbforms). While it works well, it does not generate all possible forms. This program, currently under development, aims to generate all possible forms.

Feel free to explore the repository and contribute to its development.

----------

_Note: Ensure that you have Rust installed on your system to use the programs in this repository._
