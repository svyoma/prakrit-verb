use crate::models::{Dialect, Mood};

/// Affix set for a specific mood and dialect combination
#[derive(Debug, Clone)]
pub struct AffixSet {
    pub third_singular: Vec<&'static str>,
    pub third_plural: Vec<&'static str>,
    pub second_singular: Vec<&'static str>,
    pub second_plural: Vec<&'static str>,
    pub first_singular: Vec<&'static str>,
    pub first_plural: Vec<&'static str>,
}

/// Get present tense affixes based on mood and dialect
pub fn get_present_affixes(mood: Mood, dialect: Dialect) -> AffixSet {
    match (mood, dialect) {
        // INDICATIVE MOOD
        (Mood::Indicative, Dialect::Maharastri) => AffixSet {
            third_singular: vec!["_i", "e"],
            third_plural: vec!["nti", "nte", "_ire"],
            second_singular: vec!["si", "se"],
            second_plural: vec!["ha", "_itthA"],
            first_singular: vec!["mi"],
            first_plural: vec!["mo", "mu", "ma"],
        },
        (Mood::Indicative, Dialect::Shauraseni) => AffixSet {
            third_singular: vec!["di", "de"],
            third_plural: vec!["nti", "nte", "_ire"],
            second_singular: vec!["si", "se"],
            second_plural: vec!["ha", "_itthA"],
            first_singular: vec!["mi"],
            first_plural: vec!["mo", "mu", "ma"],
        },
        (Mood::Indicative, Dialect::Magadhi) => AffixSet {
            third_singular: vec!["di", "de"],
            third_plural: vec!["nti", "nte", "_ire"],
            second_singular: vec!["zi", "ze"], // Magadhi uses 'z' instead of 's'
            second_plural: vec!["ha", "_itthA"],
            first_singular: vec!["mi"],
            first_plural: vec!["mo", "mu", "ma"],
        },
        // IMPERATIVE MOOD
        (Mood::Imperative, Dialect::Maharastri) => AffixSet {
            third_singular: vec!["_u"],
            third_plural: vec!["ntu"],
            second_singular: vec!["hi", "su"],
            second_plural: vec!["ha"],
            first_singular: vec!["mo"],
            first_plural: vec!["mu"],
        },
        (Mood::Imperative, Dialect::Shauraseni) => AffixSet {
            third_singular: vec!["du"],
            third_plural: vec!["ntu"],
            second_singular: vec!["hi", "su"],
            second_plural: vec!["ha"],
            first_singular: vec!["mo"],
            first_plural: vec!["mu"],
        },
        (Mood::Imperative, Dialect::Magadhi) => AffixSet {
            third_singular: vec!["du"],
            third_plural: vec!["ntu"],
            second_singular: vec!["hi", "zu"], // Magadhi uses 'z' instead of 's'
            second_plural: vec!["ha"],
            first_singular: vec!["mo"],
            first_plural: vec!["mu"],
        },
    }
}

/// Get future tense affixes based on dialect
pub fn get_future_affixes(dialect: Dialect) -> AffixSet {
    match dialect {
        Dialect::Maharastri => AffixSet {
            third_singular: vec!["hi_i", "hie"],
            third_plural: vec!["hinti", "hinte", "hi_ire"],
            second_singular: vec!["hisi", "hise"],
            second_plural: vec!["hitthA", "hiha"],
            first_singular: vec!["himi", "hAmi", "ssaM", "ssAmi"],
            first_plural: vec![
                "himo", "himu", "hima", "hAmo", "hAmu", "hAma", "ssAmo", "ssAmu", "ssAma",
                "hissA", "hitthA",
            ],
        },
        Dialect::Shauraseni => AffixSet {
            third_singular: vec!["hi_di", "hide"],
            third_plural: vec!["hinti", "hinte", "hi_ire"],
            second_singular: vec!["hisi", "hise"],
            second_plural: vec!["hitthA", "hiha"],
            first_singular: vec!["himi", "hAmi", "ssaM", "ssAmi"],
            first_plural: vec![
                "himo", "himu", "hima", "hAmo", "hAmu", "hAma", "ssAmo", "ssAmu", "ssAma",
                "hissA", "hitthA",
            ],
        },
        Dialect::Magadhi => AffixSet {
            third_singular: vec!["hi_di", "hide"],
            third_plural: vec!["hinti", "hinte", "hi_ire"],
            second_singular: vec!["hizi", "hize"], // Magadhi uses 'z' instead of 's'
            second_plural: vec!["hitthA", "hiha"],
            first_singular: vec!["himi", "hAmi", "ssaM", "ssAmi"],
            first_plural: vec![
                "himo", "himu", "hima", "hAmo", "hAmu", "hAma", "ssAmo", "ssAmu", "ssAma",
                "hissA", "hitthA",
            ],
        },
    }
}

/// Past tense suffixes for vowel-ending roots
pub fn get_past_suffixes_vowel() -> Vec<&'static str> {
    vec!["sI", "hI", "hIa"]
}

/// Past tense suffixes for consonant-ending roots
pub fn get_past_suffixes_consonant() -> Vec<&'static str> {
    vec!["Ia"]
}

/// Passive voice infixes
pub fn get_passive_infixes() -> Vec<&'static str> {
    vec!["ijja", "Ia"]
}
