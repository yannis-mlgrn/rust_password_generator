use inquire::{MultiSelect,CustomType};
use rand::Rng;

fn main() {
    let charset = build_charset();

    if charset.is_empty() {
        println!("Aucun type de caractère sélectionné. Abandon.");
        return;
    }

    let pass_length = CustomType::<usize>::new("How long should the password be?")
    .with_error_message("Please type a valid number")
    .prompt();

    let pass_length = match pass_length {
        Ok(len) => len,
        Err(e) => {
            println!("Erreur lors de la saisie de la longueur : {}", e);
            return;
        }
    };

    let password = build_password(&charset, pass_length);
    println!("Mot de passe généré : {}", password);
    println!("entropie : {:.2}", entropy(pass_length, charset.len()));
}

fn build_charset() -> Vec<u8> {
    let options = vec![
        "Minuscules (a-z)",
        "Majuscules (A-Z)",
        "Chiffres (0-9)",
        "Caractères spéciaux (!@#$%^&*)",
    ];

    let ans = MultiSelect::new(
        "Quels types de caractères souhaitez-vous inclure dans le mot de passe ?",
        options,
    )
    .prompt();

    let mut charset = Vec::new();
    if let Ok(selected) = ans {
        for option in selected {
            match option.as_ref() {
                "Minuscules (a-z)" => charset.extend(b'a'..=b'z'),
                "Majuscules (A-Z)" => charset.extend(b'A'..=b'Z'),
                "Chiffres (0-9)" => charset.extend(b'0'..=b'9'),
                "Caractères spéciaux (!@#$%^&*)" => {
                    charset.extend_from_slice(b"!@#$%^&*{(]}'\"|<>/?;:~`-_=+");
                }
                _ => {}
            }
        }
    }

    charset
}

fn build_password(charset: &[u8], length: usize) -> String {
    let mut rng = rand::rng();

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}


fn entropy(password_length: usize, charset_size: usize) -> f64 {
    (password_length as f64) * (charset_size as f64).log2()
}