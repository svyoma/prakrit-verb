use std::io;

fn main() {
    println!("Enter a Prakrit verb-stem/dhAtu:");
    let mut stem = String::new();
    io::stdin().read_line(&mut stem).unwrap();
    let stem = stem.trim();

    let is_vowel_ending = stem.chars().last().map(|c| "aeiou".contains(c)).unwrap_or(false);

    // Create base forms: (form, a_added, a_to_e_applied)
    let mut bases = vec![];

        let stem = stem.trim();
    let cleaned_stem = if stem.ends_with('a') {
        &stem[..stem.len() - 1] // remove final 'a'
    } else {
        stem
    };
    let is_vowel_ending = cleaned_stem.chars().last().map(|c| "aeiou".contains(c)).unwrap_or(false);

    // Use cleaned_stem from now on

    if is_vowel_ending {
        bases.push((cleaned_stem.to_string(), false, false));              
        bases.push((format!("{}a", cleaned_stem), true, false));           
        bases.push((format!("{}e", cleaned_stem), true, true));            
    } else {
        bases.push((format!("{}a", cleaned_stem), true, false));           
        bases.push((format!("{}e", cleaned_stem), true, true));            
    }
    

    // Person/Number + suffixes
    let suffix_sets = vec![
        ("Third Person Singular", vec!["i", "e"]),
        ("Third Person Plural", vec!["nti", "nte", "ire"]),
        ("Second Person Singular", vec!["si", "se"]),
        ("Second Person Plural", vec!["ha", "itthA"]),
        ("First Person Singular", vec!["mi"]),
        ("First Person Plural", vec!["mo", "mu", "ma"]),
    ];

    for (label, suffixes) in suffix_sets {
        println!("\n--- {} ---", label);
        for (base, added_a, changed_to_e) in &bases {
            for suffix in &suffixes {
                let is_e_suffix = *suffix == "e" || *suffix == "se";

                // "e"/"se" must be added only if:
                // 1. 'a' was added
                // 2. a→e was NOT applied
                if is_e_suffix {
                    if !added_a || *changed_to_e {
                        continue;
                    }
                }

                // If "a→e" was applied, skip suffixes that must follow a (like "e", "se")
                if *changed_to_e && is_e_suffix {
                    continue;
                }

                let form = format!("{}{}", base, suffix);
                println!("{}", form); // main form

                // a→A and a→i before m-suffixes
                if suffix.starts_with('m') && base.ends_with('a') {
                    let base_a = &base[..base.len() - 1];
                    println!("{}{}", base_a, format!("A{}", suffix));

                    if *suffix != "mi" {
                        println!("{}{}", base_a, format!("i{}", suffix));
                    }
                }

                // Optional shortening before consonant clusters (A→a, E→i)
                if has_consonant_cluster_start(suffix) {
                    if base.ends_with('A') {
                        let short_base = base[..base.len() - 1].to_string() + "a";
                        println!("{}{}", short_base, suffix);
                    } else if base.ends_with('e') {
                        let short_base = base[..base.len() - 1].to_string() + "i";
                        println!("{}{}", short_base, suffix);
                    }
                }
            }
        }
    }
}

fn has_consonant_cluster_start(suffix: &str) -> bool {
    let clusters = vec!["nt", "st", "tt", "tr", "kr", "pl", "pt", "sm"];
    clusters.iter().any(|c| suffix.starts_with(c))
}
