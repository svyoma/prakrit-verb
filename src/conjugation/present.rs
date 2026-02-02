use crate::conjugation::affixes::{get_passive_infixes, get_present_affixes};
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

/// A stem with metadata about whether it was derived from a consonant-ending root
#[derive(Debug, Clone)]
struct Stem {
    value: String,
    from_consonant_root: bool,
}

/// Generate stems based on root type (vowel-ending or consonant-ending)
fn generate_stems(root: &str) -> Vec<Stem> {
    if ends_with_vowel(root) {
        // Vowel-ending: optionally add 'a'
        vec![
            Stem { value: root.to_string(), from_consonant_root: false },
            Stem { value: format!("{}a", root), from_consonant_root: false },
        ]
    } else {
        // Consonant-ending: compulsorily add 'a'
        vec![Stem { value: format!("{}a", root), from_consonant_root: true }]
    }
}

/// Apply passive infixes to stems
fn apply_passive_infixes(stems: &[Stem]) -> Vec<Stem> {
    let passive_infixes = get_passive_infixes();
    let mut passive_stems = Vec::new();

    for stem in stems {
        if stem.value.ends_with('a') {
            // Remove final 'a' and add passive infixes
            let base = &stem.value[..stem.value.len() - 1];
            for infix in &passive_infixes {
                passive_stems.push(Stem {
                    value: format!("{}{}", base, infix),
                    from_consonant_root: false, // Passive stems don't need special handling
                });
            }
        } else {
            // Add passive infixes directly
            for infix in &passive_infixes {
                passive_stems.push(Stem {
                    value: format!("{}{}", stem.value, infix),
                    from_consonant_root: false,
                });
            }
        }
    }

    passive_stems
}

/// Generate e-substitution variants for stems ending in 'a'
fn generate_e_variants(stems: &[Stem]) -> Vec<Stem> {
    let mut all_stems = Vec::new();

    for stem in stems {
        if stem.value.ends_with('a') {
            all_stems.push(stem.clone());
            let base = &stem.value[..stem.value.len() - 1];
            all_stems.push(Stem {
                value: format!("{}e", base),
                from_consonant_root: stem.from_consonant_root,
            });
        } else {
            all_stems.push(stem.clone());
        }
    }

    all_stems
}

/// Remove the final vowel from a stem to get the base
fn get_base(stem: &str) -> &str {
    if stem.ends_with('a') || stem.ends_with('e') {
        &stem[..stem.len() - 1]
    } else {
        stem
    }
}

/// Generate present tense forms
pub fn generate_present_forms(
    verb_root: &str,
    voice: Voice,
    mood: Mood,
    dialect: Dialect,
) -> Result<ConjugationResult, ConjugationError> {
    if verb_root.is_empty() {
        return Err(ConjugationError::EmptyRoot);
    }

    let working_root = apply_vowel_transformation(verb_root);

    // Generate stems
    let mut stems = generate_stems(&working_root);

    // Apply passive infixes if passive voice
    if voice == Voice::Passive {
        stems = apply_passive_infixes(&stems);
    }

    // Generate e-substitution variants
    let all_stems = generate_e_variants(&stems);

    // Get affixes for this mood and dialect
    let affixes = get_present_affixes(mood, dialect);

    // Generate forms for each person
    let mut forms = PersonForms::new();

    // Third person singular
    forms.third_singular = generate_person_forms(&all_stems, &affixes.third_singular, mood, true);

    // Third person plural
    forms.third_plural = generate_person_forms(&all_stems, &affixes.third_plural, mood, false);

    // Second person singular
    forms.second_singular =
        generate_person_forms(&all_stems, &affixes.second_singular, mood, false);

    // Second person plural
    forms.second_plural = generate_person_forms(&all_stems, &affixes.second_plural, mood, false);

    // First person singular - special handling for 'mi'
    forms.first_singular =
        generate_first_singular_forms(&all_stems, &affixes.first_singular, mood);

    // First person plural - special handling for 'mo', 'mu', 'ma'
    forms.first_plural = generate_first_plural_forms(&all_stems, &affixes.first_plural, mood);

    Ok(ConjugationResult::new(
        verb_root.to_string(),
        Tense::Present,
        mood,
        voice,
        dialect,
        forms,
    ))
}

/// Generate forms for a person with standard rules
///
/// Matching Python behavior:
/// - Underscore affixes (like `_i`) are appended directly to the stem WITH the underscore
/// - The underscore is a display marker in the output (e.g., `puccha_i`)
fn generate_person_forms(
    stems: &[Stem],
    person_affixes: &[&str],
    mood: Mood,
    _is_third_singular: bool,
) -> Vec<String> {
    let mut forms = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for stem in stems {
        let base = get_base(&stem.value);

        for &affix in person_affixes {
            // Special rule: 'e' and 'se' affixes are not joined if stem doesn't end with 'a'
            if mood == Mood::Indicative && (affix == "e" || affix == "se") && !stem.value.ends_with('a') {
                continue;
            }

            // Determine the form to use
            let mut form = stem.value.clone();

            // Special rule: 'a' not changed to 'e' when followed by 'e' and 'se'
            if mood == Mood::Indicative && (affix == "e" || affix == "se") {
                if stem.value.ends_with('e') {
                    form = format!("{}a", base);
                }
            }

            // Rule for shortening long vowels before conjunct consonants
            if (mood == Mood::Indicative && (affix == "nti" || affix == "nte"))
                || (mood == Mood::Imperative && affix == "ntu")
            {
                let last_char = stem.value.chars().last().unwrap_or('a');
                if matches!(last_char, 'I' | 'U' | 'o' | 'e') {
                    // Add original form
                    let original = format!("{}{}", form, affix);
                    if seen.insert(original.clone()) {
                        forms.push(original);
                    }

                    // Add shortened form
                    let short_vowel = match last_char {
                        'I' | 'e' => 'i',
                        'U' | 'o' => 'u',
                        _ => last_char,
                    };
                    let shortened = format!("{}{}{}", base, short_vowel, affix);
                    if seen.insert(shortened.clone()) {
                        forms.push(shortened);
                    }
                    continue;
                }
            }

            // Default case: just append the affix directly (matching Python behavior)
            // Underscore affixes like `_i` are kept with the underscore in the output
            let result = format!("{}{}", form, affix);

            if seen.insert(result.clone()) {
                forms.push(result);
            }
        }
    }

    forms
}

/// Generate first person singular forms with special 'mi' handling
fn generate_first_singular_forms(stems: &[Stem], person_affixes: &[&str], mood: Mood) -> Vec<String> {
    let mut forms = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for stem in stems {
        let base = get_base(&stem.value);

        for &affix in person_affixes {
            // Special rule: 'a' -> 'A' when followed by affixes starting with 'm'
            if mood == Mood::Indicative && affix == "mi" && stem.value.ends_with('a') {
                // Regular form
                let regular = format!("{}a{}", base, affix);
                if seen.insert(regular.clone()) {
                    forms.push(regular);
                }

                // With 'A' substitution
                let with_aa = format!("{}A{}", base, affix);
                if seen.insert(with_aa.clone()) {
                    forms.push(with_aa);
                }

                // With 'e' substitution (if not already from e-substitution)
                if !stem.value.ends_with('e') {
                    let with_e = format!("{}e{}", base, affix);
                    if seen.insert(with_e.clone()) {
                        forms.push(with_e);
                    }
                }
                continue;
            }

            // Default handling
            let result = format!("{}{}", stem.value, affix);
            if seen.insert(result.clone()) {
                forms.push(result);
            }
        }
    }

    forms
}

/// Generate first person plural forms with special handling
fn generate_first_plural_forms(stems: &[Stem], person_affixes: &[&str], mood: Mood) -> Vec<String> {
    let mut forms = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for stem in stems {
        let base = get_base(&stem.value);

        for &affix in person_affixes {
            // Special rule for first person plural - four specific forms
            if mood == Mood::Indicative
                && matches!(affix, "mo" | "mu" | "ma")
                && stem.value.ends_with('a')
            {
                // Regular form
                let regular = format!("{}a{}", base, affix);
                if seen.insert(regular.clone()) {
                    forms.push(regular);
                }

                // With 'A' substitution
                let with_aa = format!("{}A{}", base, affix);
                if seen.insert(with_aa.clone()) {
                    forms.push(with_aa);
                }

                // With 'e' substitution
                let with_e = format!("{}e{}", base, affix);
                if seen.insert(with_e.clone()) {
                    forms.push(with_e);
                }

                // With 'i' substitution
                let with_i = format!("{}i{}", base, affix);
                if seen.insert(with_i.clone()) {
                    forms.push(with_i);
                }
                continue;
            }

            // Default handling
            let result = format!("{}{}", stem.value, affix);
            if seen.insert(result.clone()) {
                forms.push(result);
            }
        }
    }

    forms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('A'));
        assert!(is_vowel('i'));
        assert!(is_vowel('u'));
        assert!(!is_vowel('k'));
        assert!(!is_vowel('m'));
    }

    #[test]
    fn test_ends_with_vowel() {
        assert!(ends_with_vowel("bhU"));
        assert!(ends_with_vowel("gami"));
        assert!(!ends_with_vowel("gam"));
        assert!(!ends_with_vowel("kar"));
    }

    #[test]
    fn test_generate_stems_vowel_ending() {
        let stems = generate_stems("bhU");
        assert_eq!(stems.len(), 2);
        assert!(stems.iter().any(|s| s.value == "bhU" && !s.from_consonant_root));
        assert!(stems.iter().any(|s| s.value == "bhUa" && !s.from_consonant_root));
    }

    #[test]
    fn test_generate_stems_consonant_ending() {
        let stems = generate_stems("gam");
        assert_eq!(stems.len(), 1);
        assert!(stems.iter().any(|s| s.value == "gama" && s.from_consonant_root));
    }

    #[test]
    fn test_generate_present_forms_basic() {
        let result =
            generate_present_forms("gam", Voice::Active, Mood::Indicative, Dialect::Maharastri)
                .unwrap();
        assert_eq!(result.verb_root, "gam");
        assert!(!result.forms.third_singular.is_empty());
        assert!(!result.forms.third_plural.is_empty());
    }
}
