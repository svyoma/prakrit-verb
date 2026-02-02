pub mod affixes;
pub mod future;
pub mod past;
pub mod present;

pub use future::generate_future_forms;
pub use past::generate_past_forms;
pub use present::generate_present_forms;

use crate::error::ConjugationError;
use crate::models::{ConjugationResult, Dialect, Mood, Tense, Voice};

/// Main conjugation function that dispatches to the appropriate tense handler
pub fn conjugate(
    verb_root: &str,
    tense: Tense,
    mood: Mood,
    voice: Voice,
    dialect: Dialect,
) -> Result<ConjugationResult, ConjugationError> {
    match tense {
        Tense::Present => generate_present_forms(verb_root, voice, mood, dialect),
        Tense::Past => generate_past_forms(verb_root, voice, dialect),
        Tense::Future => generate_future_forms(verb_root, voice, dialect),
    }
}
