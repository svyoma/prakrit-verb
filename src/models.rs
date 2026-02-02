use serde::{Deserialize, Serialize};
use std::fmt;

/// Voice - Active or Passive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    #[default]
    Active,
    Passive,
}

impl fmt::Display for Voice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Voice::Active => write!(f, "active"),
            Voice::Passive => write!(f, "passive"),
        }
    }
}

/// Mood - Indicative or Imperative
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    #[default]
    Indicative,
    Imperative,
}

impl fmt::Display for Mood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mood::Indicative => write!(f, "indicative"),
            Mood::Imperative => write!(f, "imperative"),
        }
    }
}

/// Tense - Present, Past, or Future
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Tense {
    #[default]
    Present,
    Past,
    Future,
}

impl fmt::Display for Tense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tense::Present => write!(f, "present"),
            Tense::Past => write!(f, "past"),
            Tense::Future => write!(f, "future"),
        }
    }
}

/// Prakrit Dialect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Dialect {
    #[default]
    Maharastri,
    Shauraseni,
    Magadhi,
}

impl fmt::Display for Dialect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dialect::Maharastri => write!(f, "maharastri"),
            Dialect::Shauraseni => write!(f, "shauraseni"),
            Dialect::Magadhi => write!(f, "magadhi"),
        }
    }
}

/// Encoding format for input/output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    #[default]
    SLP1,
    HK,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Encoding::SLP1 => write!(f, "slp1"),
            Encoding::HK => write!(f, "hk"),
        }
    }
}

/// Forms for each grammatical person (singular and plural)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonForms {
    pub third_singular: Vec<String>,
    pub third_plural: Vec<String>,
    pub second_singular: Vec<String>,
    pub second_plural: Vec<String>,
    pub first_singular: Vec<String>,
    pub first_plural: Vec<String>,
}

impl PersonForms {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Complete conjugation result for a verb
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConjugationResult {
    pub verb_root: String,
    pub tense: Tense,
    pub mood: Mood,
    pub voice: Voice,
    pub dialect: Dialect,
    pub forms: PersonForms,
}

impl ConjugationResult {
    pub fn new(
        verb_root: String,
        tense: Tense,
        mood: Mood,
        voice: Voice,
        dialect: Dialect,
        forms: PersonForms,
    ) -> Self {
        Self {
            verb_root,
            tense,
            mood,
            voice,
            dialect,
            forms,
        }
    }
}

/// Past tense result (same forms for all persons)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastTenseResult {
    pub verb_root: String,
    pub voice: Voice,
    pub dialect: Dialect,
    pub forms: Vec<String>,
}

/// Batch processing output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOutput {
    pub results: Vec<ConjugationResult>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<BatchError>,
}

impl BatchOutput {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// Error entry for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchError {
    pub line_number: usize,
    pub verb_root: String,
    pub error_message: String,
}

/// Parameters for conjugation
#[derive(Debug, Clone)]
pub struct ConjugationParams {
    pub verb_root: String,
    pub tense: Tense,
    pub mood: Mood,
    pub voice: Voice,
    pub dialect: Dialect,
}

impl ConjugationParams {
    pub fn new(verb_root: String) -> Self {
        Self {
            verb_root,
            tense: Tense::default(),
            mood: Mood::default(),
            voice: Voice::default(),
            dialect: Dialect::default(),
        }
    }

    pub fn with_tense(mut self, tense: Tense) -> Self {
        self.tense = tense;
        self
    }

    pub fn with_mood(mut self, mood: Mood) -> Self {
        self.mood = mood;
        self
    }

    pub fn with_voice(mut self, voice: Voice) -> Self {
        self.voice = voice;
        self
    }

    pub fn with_dialect(mut self, dialect: Dialect) -> Self {
        self.dialect = dialect;
        self
    }
}
