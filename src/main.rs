use std::io::{self, BufRead};

fn main() {
    // Read inputs
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        match line {
            Ok(input) => {
                println!("{}", convert(input))
            }
            Err(e) => panic!("error in input: {}", e),
        }
    }
}

fn convert(s: String) -> String {
    s.chars().map(|c| convert_char(c)).collect::<String>()
}

fn convert_char(c: char) -> char {
    match c {
        'a'..='z' => convert_basic(c),
        'A'..='Z' => convert_basic(c.to_ascii_lowercase()).to_ascii_uppercase(),
        _ => convert_tilde(c),
    }
}

fn convert_basic(c: char) -> char {
    match c {
        // Vowels
        'a' => 'e',
        'e' => 'a',
        'i' => 'o',
        'o' => 'i',

        // Consonants
        'c' => 'v',
        'f' => 'g',
        'g' => 'f',
        'l' => 't',
        'm' => 'n',
        'n' => 'm',
        'r' => 's',
        's' => 'r',
        't' => 'l',
        'v' => 'c',

        // Other letters
        _ => c,
    }
}

fn convert_tilde(c: char) -> char {
    match c {
        // minúsculas
        'á' => 'é',
        'é' => 'á',
        'í' => 'ó',
        'ó' => 'í',

        // MAYÚSCULAS
        'Á' => 'É',
        'É' => 'Á',
        'Í' => 'Ó',
        'Ó' => 'Í',

        // los demas
        _ => c,
    }
}
