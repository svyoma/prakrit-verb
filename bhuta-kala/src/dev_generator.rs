use std::io;

fn main() {
    println!("Welcome to Pāia language bhūta-kāla form generator. This tool uses devanāgarī script as default scheme. Please enter a dhātu.");

    // Create a mutable string to store user input
    let mut user_input = String::new();

    // Read user input from the console
    io::stdin().read_line(&mut user_input)
        .expect("Failed to read line");

    // Trims whitespace and newline characters from the input
    let user_input = user_input.trim();
    
    // Remove '्' from the end of user input if available
    let user_input = if user_input.ends_with("्") {
        &user_input[..user_input.len() - 3]
    } else {
        user_input
    };

    // Checks if the user entered a word
    if !user_input.is_empty() {
        // Determines if the input ends with a vowel
        let ends_with_vowel = user_input.chars().last().map_or(false, |c| "ािीुू".contains(c));

        // Generates forms
        println!("You have entered: {}", user_input);
        println!("Its forms in bhūta-kāla are as follows:");

        if ends_with_vowel {
            println!("As entered dhātu is svarānta, through 'सी-ही-हीअ भूतार्थस्य॥८।३।१६२॥', its forms are:");
            generate_result(user_input, "सी");
            generate_result(user_input, "ही");
            generate_result(user_input, "हीअ");
        } else {
            println!("As entered dhātu is vyañjanānta, through 'व्यञ्जनादीअः॥८।३।१६३॥', its form is:");
            generate_result(user_input, "ीअ");
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