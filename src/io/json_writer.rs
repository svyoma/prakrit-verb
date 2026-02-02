use crate::error::Result;
use crate::models::{BatchOutput, ConjugationResult};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Write batch output to a JSON file
pub fn write_json_file(output: &BatchOutput, path: &Path) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, output)?;
    Ok(())
}

/// Format a single conjugation result as JSON string
pub fn format_json(result: &ConjugationResult) -> Result<String> {
    Ok(serde_json::to_string_pretty(result)?)
}

/// Format batch output as JSON string
pub fn format_batch_json(output: &BatchOutput) -> Result<String> {
    Ok(serde_json::to_string_pretty(output)?)
}

/// Write JSON to stdout
pub fn write_json_stdout(result: &ConjugationResult) -> Result<()> {
    let json = format_json(result)?;
    println!("{}", json);
    Ok(())
}

/// Write batch JSON to a writer
pub fn write_batch_json<W: Write>(output: &BatchOutput, writer: &mut W) -> Result<()> {
    serde_json::to_writer_pretty(writer, output)?;
    Ok(())
}
