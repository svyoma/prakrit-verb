# prakrit-verb
A repository containing rust programmes for generating prakrit verb forms.

## bhUtakAla form generator

bhuta-kala directory contains Prākṛta Past Tense form generator. It is based on two sūtras of Hemacandrācārya "sī-hī-hīa bhūtārthasya 8.3.162" and "vyañjanādīyaḥ 8.3.163". It is based on user-input and might generate forms which do not exist in Prākṛta owing to oversight of usage of Prākṛta of user.

### Usage

There is no front-end interface for using this. To use 
- Clone this repository.
- rustc dev_generator.rs or slp_generator.rs as required.
- run the compiled .exe file

Alternatively, you can copy code from dev_generator.rs or slp_generator.rs and run on [Rust Playground](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2024)

## vartamAna form generator

Karnatak Samskrit University has developed and deployed a present tense verb form generator in python on their [Sambhāṣa platform](https://sambhasha.ksu.ac.in/CompLing/prakrit_verbforms). It works pretty well, but doesn't generate all the possible forms. This program, under development, shall generate all possible forms.