use crate::error::Result;
use crate::models::{BatchOutput, ConjugationResult};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// CSV headers for conjugation output
const CSV_HEADERS: &[&str] = &[
    "verb_root",
    "tense",
    "mood",
    "voice",
    "dialect",
    "person",
    "number",
    "forms",
];

/// Write batch output to a CSV file
pub fn write_csv_file(output: &BatchOutput, path: &Path) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = csv::Writer::from_writer(file);

    // Write headers
    writer.write_record(CSV_HEADERS)?;

    // Write each result
    for result in &output.results {
        write_result_rows(&mut writer, result)?;
    }

    writer.flush()?;
    Ok(())
}

/// Write a single conjugation result as CSV rows
fn write_result_rows<W: Write>(
    writer: &mut csv::Writer<W>,
    result: &ConjugationResult,
) -> Result<()> {
    let verb_root = &result.verb_root;
    let tense = result.tense.to_string();
    let mood = result.mood.to_string();
    let voice = result.voice.to_string();
    let dialect = result.dialect.to_string();

    // Third person singular
    writer.write_record(&[
        verb_root,
        &tense,
        &mood,
        &voice,
        &dialect,
        "third",
        "singular",
        &result.forms.third_singular.join(", "),
    ])?;

    // Third person plural
    writer.write_record(&[
        verb_root,
        &tense,
        &mood,
        &voice,
        &dialect,
        "third",
        "plural",
        &result.forms.third_plural.join(", "),
    ])?;

    // Second person singular
    writer.write_record(&[
        verb_root,
        &tense,
        &mood,
        &voice,
        &dialect,
        "second",
        "singular",
        &result.forms.second_singular.join(", "),
    ])?;

    // Second person plural
    writer.write_record(&[
        verb_root,
        &tense,
        &mood,
        &voice,
        &dialect,
        "second",
        "plural",
        &result.forms.second_plural.join(", "),
    ])?;

    // First person singular
    writer.write_record(&[
        verb_root,
        &tense,
        &mood,
        &voice,
        &dialect,
        "first",
        "singular",
        &result.forms.first_singular.join(", "),
    ])?;

    // First person plural
    writer.write_record(&[
        verb_root,
        &tense,
        &mood,
        &voice,
        &dialect,
        "first",
        "plural",
        &result.forms.first_plural.join(", "),
    ])?;

    Ok(())
}

/// Format a single conjugation result as CSV string
pub fn format_csv(result: &ConjugationResult) -> Result<String> {
    let mut writer = csv::Writer::from_writer(vec![]);

    // Write headers
    writer.write_record(CSV_HEADERS)?;

    // Write result rows
    write_result_rows(&mut writer, result)?;

    let data = writer.into_inner().map_err(|e| {
        crate::error::AppError::InvalidInput(format!("CSV write error: {}", e.into_error()))
    })?;

    Ok(String::from_utf8_lossy(&data).to_string())
}

/// Write CSV to stdout
pub fn write_csv_stdout(result: &ConjugationResult) -> Result<()> {
    let csv = format_csv(result)?;
    print!("{}", csv);
    Ok(())
}
