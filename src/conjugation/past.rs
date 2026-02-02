use crate::conjugation::affixes::{get_passive_infixes, get_past_suffixes_consonant, get_past_suffixes_vowel};
use crate::error::ConjugationError;
use crate::models::{ConjugationResult, Dialect, Mood, PersonForms, Tense, Voice};
use rand::Rng;

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

/// Generate past tense forms
/// In Prakrit, past tense forms are the same for all persons and numbers
pub fn generate_past_forms(
    verb_root: &str,
    voice: Voice,
    dialect: Dialect,
) -> Result<ConjugationResult, ConjugationError> {
    if verb_root.is_empty() {
        return Err(ConjugationError::EmptyRoot);
    }

    // Determine if the verb root ends with a vowel BEFORE transformation
    let original_ends_with_vowel = ends_with_vowel(verb_root);

    // Apply vowel transformation
    let working_root = apply_vowel_transformation(verb_root);

    // Generate past forms based on the verb ending
    let past_forms: Vec<String> = if original_ends_with_vowel {
        // For vowel-ending roots, apply sI, hI, hIa suffixes
        // (sī-hī-hīa bhūtārthasya 8.3.162)
        get_past_suffixes_vowel()
            .iter()
            .map(|suffix| format!("{}{}", working_root, suffix))
            .collect()
    } else {
        // For consonant-ending roots, apply Ia suffix
        // (vyañjanādīaḥ 8.3.163)
        get_past_suffixes_consonant()
            .iter()
            .map(|suffix| format!("{}{}", working_root, suffix))
            .collect()
    };

    // Apply passive voice if needed
    let final_forms = if voice == Voice::Passive {
        apply_passive_to_past(&past_forms)
    } else {
        past_forms
    };

    // In Prakrit, past tense forms are identical for all persons and numbers
    let forms = PersonForms {
        third_singular: final_forms.clone(),
        third_plural: final_forms.clone(),
        second_singular: final_forms.clone(),
        second_plural: final_forms.clone(),
        first_singular: final_forms.clone(),
        first_plural: final_forms,
    };

    Ok(ConjugationResult::new(
        verb_root.to_string(),
        Tense::Past,
        Mood::Indicative, // Past tense is always indicative
        voice,
        dialect,
        forms,
    ))
}

/// Apply passive infixes to past tense forms
fn apply_passive_to_past(past_forms: &[String]) -> Vec<String> {
    let passive_infixes = get_passive_infixes();
    let mut passive_forms = Vec::new();

    for form in past_forms {
        // For consonant-ending roots with 'Ia' suffix
        if form.ends_with("Ia") {
            let base = &form[..form.len() - 2];
            for infix in &passive_infixes {
                passive_forms.push(format!("{}{}", base, infix));
            }
        }
        // For vowel-ending roots with sI suffix
        else if form.ends_with("sI") {
            let base = &form[..form.len() - 2];
            for infix in &passive_infixes {
                passive_forms.push(format!("{}{}sI", base, infix));
            }
        }
        // For vowel-ending roots with hI suffix
        else if form.ends_with("hI") && !form.ends_with("hIa") {
            let base = &form[..form.len() - 2];
            for infix in &passive_infixes {
                passive_forms.push(format!("{}{}hI", base, infix));
            }
        }
        // For vowel-ending roots with hIa suffix
        else if form.ends_with("hIa") {
            let base = &form[..form.len() - 3];
            for infix in &passive_infixes {
                passive_forms.push(format!("{}{}hIa", base, infix));
            }
        }
        // Default case: just add passive infixes
        else {
            for infix in &passive_infixes {
                passive_forms.push(format!("{}{}", form, infix));
            }
        }
    }

    passive_forms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_past_vowel_ending() {
        let result = generate_past_forms("bhU", Voice::Active, Dialect::Maharastri).unwrap();
        // After vowel transformation, bhU might become bho
        // Past forms should include suffixes sI, hI, hIa
        assert!(!result.forms.third_singular.is_empty());
        // All persons should have the same forms
        assert_eq!(result.forms.third_singular, result.forms.first_singular);
    }

    #[test]
    fn test_generate_past_consonant_ending() {
        let result = generate_past_forms("gam", Voice::Active, Dialect::Maharastri).unwrap();
        // Consonant-ending roots get 'Ia' suffix
        assert!(result.forms.third_singular.iter().any(|f| f.ends_with("Ia")));
    }

    #[test]
    fn test_generate_past_passive() {
        let result = generate_past_forms("gam", Voice::Passive, Dialect::Maharastri).unwrap();
        // Passive forms should have passive infixes
        assert!(result
            .forms
            .third_singular
            .iter()
            .any(|f| f.contains("ijja") || f.contains("Ia")));
    }
}
