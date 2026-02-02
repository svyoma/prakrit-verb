use crate::conjugation::affixes::{get_future_affixes, get_passive_infixes};
use crate::error::ConjugationError;
use crate::models::{ConjugationResult, Dialect, Mood, PersonForms, Tense, Voice};
use rand::Rng;
use std::collections::HashSet;

const VOWELS: &str = "aeiouAEIOU";

/// Check if a character is a vowel
fn is_vowel(ch: char) -> bool {
    VOWELS.contains(ch)
}

/// Check if root ends with a vowel
fn ends_with_vowel(root: &str) -> bool {
    root.chars().last().map_or(false, is_vowel)
}

/// Apply vowel transformation rule: i/I → e, u/U → o
/// With 19/20 probability (exception in 1/20 cases)
fn apply_vowel_transformation(root: &str) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = root.chars().collect();

    if chars.is_empty() {
        return root.to_string();
    }

    let last = chars[chars.len() - 1];

    // Check if last character is i, I, u, or U
    if matches!(last, 'i' | 'I') {
        // 19/20 chance to apply the rule
        if rng.gen_range(1..=20) != 1 {
            let mut result: String = chars[..chars.len() - 1].iter().collect();
            result.push('e');
            return result;
        }
    } else if matches!(last, 'u' | 'U') {
        // 19/20 chance to apply the rule
        if rng.gen_range(1..=20) != 1 {
            let mut result: String = chars[..chars.len() - 1].iter().collect();
            result.push('o');
            return result;
        }
    }

    root.to_string()
}

/// Generate stems for future tense
/// Different from present: consonant-ending roots add 'i' or 'e' instead of 'a'
fn generate_future_stems(root: &str) -> Vec<String> {
    if ends_with_vowel(root) {
        // Vowel-ending: optionally add 'a'
        vec![root.to_string(), format!("{}a", root)]
    } else {
        // Consonant-ending: add 'i' and 'e' (not 'a')
        vec![format!("{}i", root), format!("{}e", root)]
    }
}

/// Apply passive infixes to stems
fn apply_passive_infixes(stems: &[String]) -> Vec<String> {
    let passive_infixes = get_passive_infixes();
    let mut passive_stems = Vec::new();

    for stem in stems {
        let last_char = stem.chars().last().unwrap_or('a');
        if matches!(last_char, 'a' | 'i' | 'e') {
            // Remove final vowel and add passive infixes
            let base = &stem[..stem.len() - 1];
            for infix in &passive_infixes {
                passive_stems.push(format!("{}{}", base, infix));
            }
        } else {
            // Add passive infixes directly
            for infix in &passive_infixes {
                passive_stems.push(format!("{}{}", stem, infix));
            }
        }
    }

    passive_stems
}

/// Generate future tense forms
pub fn generate_future_forms(
    verb_root: &str,
    voice: Voice,
    dialect: Dialect,
) -> Result<ConjugationResult, ConjugationError> {
    if verb_root.is_empty() {
        return Err(ConjugationError::EmptyRoot);
    }

    let working_root = apply_vowel_transformation(verb_root);
    let original_ends_with_vowel = ends_with_vowel(&working_root);

    // Generate stems
    let mut stems = generate_future_stems(&working_root);

    // Apply passive infixes if passive voice
    if voice == Voice::Passive {
        stems = apply_passive_infixes(&stems);
    }

    // Get affixes for this dialect
    let affixes = get_future_affixes(dialect);

    // Generate forms for each person
    let mut forms = PersonForms::new();

    forms.third_singular = generate_future_person_forms(
        &stems,
        &affixes.third_singular,
        &working_root,
        original_ends_with_vowel,
        voice,
    );
    forms.third_plural = generate_future_person_forms(
        &stems,
        &affixes.third_plural,
        &working_root,
        original_ends_with_vowel,
        voice,
    );
    forms.second_singular = generate_future_person_forms(
        &stems,
        &affixes.second_singular,
        &working_root,
        original_ends_with_vowel,
        voice,
    );
    forms.second_plural = generate_future_person_forms(
        &stems,
        &affixes.second_plural,
        &working_root,
        original_ends_with_vowel,
        voice,
    );
    forms.first_singular = generate_future_person_forms(
        &stems,
        &affixes.first_singular,
        &working_root,
        original_ends_with_vowel,
        voice,
    );
    forms.first_plural = generate_future_person_forms(
        &stems,
        &affixes.first_plural,
        &working_root,
        original_ends_with_vowel,
        voice,
    );

    Ok(ConjugationResult::new(
        verb_root.to_string(),
        Tense::Future,
        Mood::Indicative, // Future tense uses indicative mood
        voice,
        dialect,
        forms,
    ))
}

/// Generate forms for a person in future tense
/// Matching Python behavior: underscores in affixes are kept in output
fn generate_future_person_forms(
    stems: &[String],
    person_affixes: &[&str],
    working_root: &str,
    original_ends_with_vowel: bool,
    voice: Voice,
) -> Vec<String> {
    let mut forms = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for stem in stems {
        for &affix in person_affixes {
            // For vowel-ending roots with optional 'a'
            if original_ends_with_vowel && voice == Voice::Active {
                if stem == working_root {
                    // Without 'a' - append affix directly
                    let form = format!("{}{}", stem, affix);
                    if seen.insert(form.clone()) {
                        forms.push(form);
                    }
                } else if stem.ends_with('a') {
                    // With 'a' - change to 'i' and 'e' variants
                    let base = &stem[..stem.len() - 1];

                    let form_i = format!("{}i{}", base, affix);
                    if seen.insert(form_i.clone()) {
                        forms.push(form_i);
                    }

                    let form_e = format!("{}e{}", base, affix);
                    if seen.insert(form_e.clone()) {
                        forms.push(form_e);
                    }
                }
            } else {
                // Consonant-ending roots or passive voice - just append affix
                let form = format!("{}{}", stem, affix);
                if seen.insert(form.clone()) {
                    forms.push(form);
                }
            }
        }
    }

    forms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_future_stems_vowel_ending() {
        let stems = generate_future_stems("bhU");
        assert_eq!(stems.len(), 2);
        assert!(stems.contains(&"bhU".to_string()));
        assert!(stems.contains(&"bhUa".to_string()));
    }

    #[test]
    fn test_generate_future_stems_consonant_ending() {
        let stems = generate_future_stems("gam");
        assert_eq!(stems.len(), 2);
        assert!(stems.contains(&"gami".to_string()));
        assert!(stems.contains(&"game".to_string()));
    }

    #[test]
    fn test_generate_future_forms_basic() {
        let result =
            generate_future_forms("gam", Voice::Active, Dialect::Maharastri).unwrap();
        assert_eq!(result.verb_root, "gam");
        assert!(!result.forms.third_singular.is_empty());
        // Future forms should have 'hi' prefix pattern
        assert!(result
            .forms
            .third_singular
            .iter()
            .any(|f| f.contains("hi")));
    }
}
