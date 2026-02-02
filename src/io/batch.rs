use crate::conjugation::conjugate;
use crate::error::{AppError, Result};
use crate::models::{BatchError, BatchOutput, ConjugationResult, Dialect, Mood, Tense, Voice};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A tense-mood pair for batch processing
#[derive(Debug, Clone, Copy)]
pub struct TenseMood {
    pub tense: Tense,
    pub mood: Mood,
}

/// Batch processor configuration
pub struct BatchProcessor {
    pub tense_moods: Vec<TenseMood>,
    pub voices: Vec<Voice>,
    pub dialects: Vec<Dialect>,
}

impl BatchProcessor {
    /// Create a new batch processor with default settings (present indicative, active, maharastri)
    pub fn new() -> Self {
        Self {
            tense_moods: vec![TenseMood { tense: Tense::Present, mood: Mood::Indicative }],
            voices: vec![Voice::Active],
            dialects: vec![Dialect::Maharastri],
        }
    }

    /// Set specific tense-mood combinations
    pub fn with_tense_moods(mut self, tense_moods: Vec<TenseMood>) -> Self {
        if !tense_moods.is_empty() {
            self.tense_moods = tense_moods;
        }
        self
    }

    /// Set specific voices
    pub fn with_voices(mut self, voices: Vec<Voice>) -> Self {
        if !voices.is_empty() {
            self.voices = voices;
        }
        self
    }

    /// Set specific dialects
    pub fn with_dialects(mut self, dialects: Vec<Dialect>) -> Self {
        if !dialects.is_empty() {
            self.dialects = dialects;
        }
        self
    }

    /// Set all tenses (present, past, future, imperative)
    pub fn with_all_tenses(mut self) -> Self {
        self.tense_moods = vec![
            TenseMood { tense: Tense::Present, mood: Mood::Indicative },
            TenseMood { tense: Tense::Past, mood: Mood::Indicative },
            TenseMood { tense: Tense::Future, mood: Mood::Indicative },
            TenseMood { tense: Tense::Present, mood: Mood::Imperative },
        ];
        self
    }

    /// Set all dialects
    pub fn with_all_dialects(mut self) -> Self {
        self.dialects = vec![Dialect::Maharastri, Dialect::Shauraseni, Dialect::Magadhi];
        self
    }

    /// Set all voices
    pub fn with_all_voices(mut self) -> Self {
        self.voices = vec![Voice::Active, Voice::Passive];
        self
    }

    /// Process a batch file containing verb roots
    pub fn process_file(&self, path: &Path) -> Result<BatchOutput> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut output = BatchOutput::new();

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let verb_root = line.trim();

            // Skip empty lines and comments
            if verb_root.is_empty() || verb_root.starts_with('#') {
                continue;
            }

            match self.conjugate_verb(verb_root) {
                Ok(results) => output.results.extend(results),
                Err(e) => output.errors.push(BatchError {
                    line_number: line_num + 1,
                    verb_root: verb_root.to_string(),
                    error_message: e.to_string(),
                }),
            }
        }

        Ok(output)
    }

    /// Conjugate a single verb with all configured combinations
    fn conjugate_verb(&self, verb_root: &str) -> Result<Vec<ConjugationResult>> {
        let mut results = Vec::new();

        // Generate cartesian product: tense_moods × voices × dialects
        for tense_mood in &self.tense_moods {
            for voice in &self.voices {
                for dialect in &self.dialects {
                    let result = conjugate(
                        verb_root,
                        tense_mood.tense,
                        tense_mood.mood,
                        *voice,
                        *dialect,
                    )
                    .map_err(AppError::from)?;
                    results.push(result);
                }
            }
        }

        Ok(results)
    }
}

impl Default for BatchProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_batch_processor_single_verb() {
        let processor = BatchProcessor::new();
        let results = processor.conjugate_verb("gam").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].verb_root, "gam");
    }

    #[test]
    fn test_batch_processor_all_tenses() {
        let processor = BatchProcessor::new().with_all_tenses();
        let results = processor.conjugate_verb("gam").unwrap();
        assert_eq!(results.len(), 4); // Present, Past, Future, Imperative
    }

    #[test]
    fn test_batch_processor_all_dialects() {
        let processor = BatchProcessor::new().with_all_dialects();
        let results = processor.conjugate_verb("gam").unwrap();
        assert_eq!(results.len(), 3); // Maharastri, Shauraseni, Magadhi
    }

    #[test]
    fn test_batch_processor_all_voices() {
        let processor = BatchProcessor::new().with_all_voices();
        let results = processor.conjugate_verb("gam").unwrap();
        assert_eq!(results.len(), 2); // Active, Passive
    }

    #[test]
    fn test_batch_processor_combinations() {
        // Test 2 tenses × 2 dialects × 2 voices = 8 combinations
        let processor = BatchProcessor::new()
            .with_tense_moods(vec![
                TenseMood { tense: Tense::Present, mood: Mood::Indicative },
                TenseMood { tense: Tense::Future, mood: Mood::Indicative },
            ])
            .with_dialects(vec![Dialect::Maharastri, Dialect::Shauraseni])
            .with_voices(vec![Voice::Active, Voice::Passive]);
        let results = processor.conjugate_verb("gam").unwrap();
        assert_eq!(results.len(), 8);
    }

    #[test]
    fn test_batch_processor_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "gam").unwrap();
        writeln!(temp_file, "# This is a comment").unwrap();
        writeln!(temp_file, "bhU").unwrap();
        writeln!(temp_file, "").unwrap(); // Empty line

        let processor = BatchProcessor::new();
        let output = processor.process_file(temp_file.path()).unwrap();

        assert_eq!(output.results.len(), 2);
        assert!(output.errors.is_empty());
    }
}
