use clap::Parser;
use prakrit_verb_cli::cli::{BatchOutputFormat, Cli, Commands, OutputFormat};
use prakrit_verb_cli::conjugation::conjugate;
use prakrit_verb_cli::encoding::format_forms;
use prakrit_verb_cli::error::Result;
use prakrit_verb_cli::io::{write_csv_file, write_csv_stdout, write_json_file, write_json_stdout, BatchProcessor, TenseMood};
use prakrit_verb_cli::models::{BatchOutput, ConjugationResult, Dialect, Encoding, Mood, Tense, Voice};
use std::io::{self, BufRead, Write};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Conjugate {
            verb,
            tense,
            voice,
            dialect,
            format,
            encoding,
            output,
        } => {
            let tense_enum: Tense = tense.into();
            let mood = if tense.is_imperative() {
                Mood::Imperative
            } else {
                Mood::Indicative
            };
            let voice_enum: Voice = voice.into();
            let dialect_enum: Dialect = dialect.into();
            let encoding_enum: Encoding = encoding.into();

            let mut result = conjugate(&verb, tense_enum, mood, voice_enum, dialect_enum)?;

            // Apply encoding conversion if needed
            result = apply_encoding(result, encoding_enum);

            // Output based on format
            match format {
                OutputFormat::Table => {
                    if let Some(path) = output {
                        let content = format_table(&result);
                        std::fs::write(path, content)?;
                    } else {
                        print_table(&result);
                    }
                }
                OutputFormat::Json => {
                    if let Some(path) = output {
                        let batch = BatchOutput {
                            results: vec![result],
                            errors: vec![],
                        };
                        write_json_file(&batch, &path)?;
                    } else {
                        write_json_stdout(&result)?;
                    }
                }
                OutputFormat::Csv => {
                    if let Some(path) = output {
                        let batch = BatchOutput {
                            results: vec![result],
                            errors: vec![],
                        };
                        write_csv_file(&batch, &path)?;
                    } else {
                        write_csv_stdout(&result)?;
                    }
                }
            }
        }

        Commands::Batch {
            input,
            output,
            format,
            encoding,
            tenses,
            voices,
            dialects,
            all_tenses,
            all_dialects,
            all_voices,
            all,
        } => {
            let encoding_enum: Encoding = encoding.into();

            // Build tense-mood combinations from CLI args
            let tense_moods: Vec<TenseMood> = tenses
                .iter()
                .map(|t| {
                    let tense: Tense = (*t).into();
                    let mood = if t.is_imperative() {
                        Mood::Imperative
                    } else {
                        Mood::Indicative
                    };
                    TenseMood { tense, mood }
                })
                .collect();

            // Build voices from CLI args
            let voice_list: Vec<Voice> = voices.iter().map(|v| (*v).into()).collect();

            // Build dialects from CLI args
            let dialect_list: Vec<Dialect> = dialects.iter().map(|d| (*d).into()).collect();

            // Create processor with specified options
            let mut processor = BatchProcessor::new()
                .with_tense_moods(tense_moods)
                .with_voices(voice_list)
                .with_dialects(dialect_list);

            // Apply "all" flags
            if all || all_tenses {
                processor = processor.with_all_tenses();
            }
            if all || all_dialects {
                processor = processor.with_all_dialects();
            }
            if all || all_voices {
                processor = processor.with_all_voices();
            }

            let mut batch_output = processor.process_file(&input)?;

            // Apply encoding conversion to all results
            batch_output.results = batch_output
                .results
                .into_iter()
                .map(|r| apply_encoding(r, encoding_enum))
                .collect();

            // Write output
            match format {
                BatchOutputFormat::Json => {
                    write_json_file(&batch_output, &output)?;
                    println!("Output written to: {}", output.display());
                    println!(
                        "Processed {} conjugations, {} errors",
                        batch_output.results.len(),
                        batch_output.errors.len()
                    );
                }
                BatchOutputFormat::Csv => {
                    write_csv_file(&batch_output, &output)?;
                    println!("Output written to: {}", output.display());
                    println!(
                        "Processed {} conjugations, {} errors",
                        batch_output.results.len(),
                        batch_output.errors.len()
                    );
                }
            }

            // Print errors if any
            if !batch_output.errors.is_empty() {
                eprintln!("\nErrors:");
                for error in &batch_output.errors {
                    eprintln!(
                        "  Line {}: {} - {}",
                        error.line_number, error.verb_root, error.error_message
                    );
                }
            }
        }

        Commands::Interactive => {
            run_interactive_mode()?;
        }
    }

    Ok(())
}

/// Apply encoding conversion to conjugation result
fn apply_encoding(mut result: ConjugationResult, encoding: Encoding) -> ConjugationResult {
    result.forms.third_singular = format_forms(&result.forms.third_singular, encoding);
    result.forms.third_plural = format_forms(&result.forms.third_plural, encoding);
    result.forms.second_singular = format_forms(&result.forms.second_singular, encoding);
    result.forms.second_plural = format_forms(&result.forms.second_plural, encoding);
    result.forms.first_singular = format_forms(&result.forms.first_singular, encoding);
    result.forms.first_plural = format_forms(&result.forms.first_plural, encoding);
    result
}

/// Print conjugation result as a human-readable table
fn print_table(result: &ConjugationResult) {
    println!("Verb Root: {}", result.verb_root);
    println!("Tense: {}", result.tense);
    println!("Mood: {}", result.mood);
    println!("Voice: {}", result.voice);
    println!("Dialect: {}", result.dialect);
    println!();
    println!(
        "{:<20} {:<40} {:<40}",
        "Person", "Singular", "Plural"
    );
    println!("{}", "-".repeat(100));
    println!(
        "{:<20} {:<40} {:<40}",
        "Third Person",
        result.forms.third_singular.join(", "),
        result.forms.third_plural.join(", ")
    );
    println!(
        "{:<20} {:<40} {:<40}",
        "Second Person",
        result.forms.second_singular.join(", "),
        result.forms.second_plural.join(", ")
    );
    println!(
        "{:<20} {:<40} {:<40}",
        "First Person",
        result.forms.first_singular.join(", "),
        result.forms.first_plural.join(", ")
    );
}

/// Format conjugation result as a table string
fn format_table(result: &ConjugationResult) -> String {
    let mut output = String::new();
    output.push_str(&format!("Verb Root: {}\n", result.verb_root));
    output.push_str(&format!("Tense: {}\n", result.tense));
    output.push_str(&format!("Mood: {}\n", result.mood));
    output.push_str(&format!("Voice: {}\n", result.voice));
    output.push_str(&format!("Dialect: {}\n", result.dialect));
    output.push('\n');
    output.push_str(&format!(
        "{:<20} {:<40} {:<40}\n",
        "Person", "Singular", "Plural"
    ));
    output.push_str(&format!("{}\n", "-".repeat(100)));
    output.push_str(&format!(
        "{:<20} {:<40} {:<40}\n",
        "Third Person",
        result.forms.third_singular.join(", "),
        result.forms.third_plural.join(", ")
    ));
    output.push_str(&format!(
        "{:<20} {:<40} {:<40}\n",
        "Second Person",
        result.forms.second_singular.join(", "),
        result.forms.second_plural.join(", ")
    ));
    output.push_str(&format!(
        "{:<20} {:<40} {:<40}\n",
        "First Person",
        result.forms.first_singular.join(", "),
        result.forms.first_plural.join(", ")
    ));
    output
}

/// Run interactive mode
fn run_interactive_mode() -> Result<()> {
    println!("Prakrit Verb Conjugation - Interactive Mode");
    println!("============================================");
    println!("Commands:");
    println!("  <verb> [tense] [dialect] [voice]  - Conjugate a verb");
    println!("  tenses: present (default), past, future, imperative");
    println!("  dialects: maharastri (default), shauraseni, magadhi");
    println!("  voices: active (default), passive");
    println!("  help - Show this help");
    println!("  quit - Exit");
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input.to_lowercase().as_str() {
            "quit" | "exit" | "q" => {
                println!("Goodbye!");
                break;
            }
            "help" | "h" | "?" => {
                println!("Commands:");
                println!("  <verb> [tense] [dialect] [voice]  - Conjugate a verb");
                println!("  tenses: present (default), past, future, imperative");
                println!("  dialects: maharastri (default), shauraseni, magadhi");
                println!("  voices: active (default), passive");
                println!("  help - Show this help");
                println!("  quit - Exit");
                continue;
            }
            _ => {}
        }

        // Parse input
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let verb = parts[0];
        let tense = parts.get(1).map(|s| parse_tense(s)).unwrap_or(Tense::Present);
        let mood = parts
            .get(1)
            .map(|s| {
                if s.to_lowercase() == "imperative" {
                    Mood::Imperative
                } else {
                    Mood::Indicative
                }
            })
            .unwrap_or(Mood::Indicative);
        let dialect = parts
            .get(2)
            .map(|s| parse_dialect(s))
            .unwrap_or(Dialect::Maharastri);
        let voice = parts.get(3).map(|s| parse_voice(s)).unwrap_or(Voice::Active);

        match conjugate(verb, tense, mood, voice, dialect) {
            Ok(result) => {
                println!();
                print_table(&result);
                println!();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}

fn parse_tense(s: &str) -> Tense {
    match s.to_lowercase().as_str() {
        "present" => Tense::Present,
        "past" => Tense::Past,
        "future" => Tense::Future,
        "imperative" => Tense::Present, // Imperative uses present tense logic
        _ => Tense::Present,
    }
}

fn parse_dialect(s: &str) -> Dialect {
    match s.to_lowercase().as_str() {
        "maharastri" => Dialect::Maharastri,
        "shauraseni" => Dialect::Shauraseni,
        "magadhi" => Dialect::Magadhi,
        _ => Dialect::Maharastri,
    }
}

fn parse_voice(s: &str) -> Voice {
    match s.to_lowercase().as_str() {
        "active" => Voice::Active,
        "passive" => Voice::Passive,
        _ => Voice::Active,
    }
}
