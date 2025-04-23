use std::io;

fn main() {
    println!("Welcome to prAkRta bhUtakAla form generator. This programme uses SLP1/HK as default transliteration scheme. Enter a dhAtu in SLP1/HK:");

    // Create a mutable string to store user input
    let mut user_input = String::new();

    // Read user input from the console
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    // Trims whitespace and newline characters from the input
    let user_input = user_input.trim();
    
    // Checks if the user entered a word
    if !user_input.is_empty() {
        // Determines if the input ends with a vowel
        let ends_with_vowel = user_input.chars().last().map_or(false, |c| "aeiouAEIOU".contains(c));

        // Generates forms
        println!("You have entered: {}", user_input);
        println!("Its forms in bhUta-kAla are as follows:");

        if ends_with_vowel {
            println!("As entered dhātu is svarAnta, through 'sI-hI-hIa bhUtArthasya 8.3.162', its forms are:");
            generate_result(user_input, "sI");
            generate_result(user_input, "hI");
            generate_result(user_input, "hIa");
        } else {
            println!("As entered dhātu is vyaJjanAnta, through 'vyaJjanAdIaH 8.3.163', its form is:");
            generate_result(user_input, "Ia");
        }
    } else {
        println!("It seems you have not entered anything. Exiting...");
    }
}

fn generate_result(input: &str, suffix: &str) {
    // Concatenate the suffix to the user input
    let result = format!("{}{}", input, suffix);

    // Print the generated result
    println!("{}", result);
}