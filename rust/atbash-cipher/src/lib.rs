/// "Encipher" with the Atbash cipher.
fn ord(c: char) -> u8 {
    match c {
        o if o.is_uppercase() => c as u8 - b'A',
        o if o.is_lowercase() => c as u8 - b'a',
        _ => panic!("Only accept alphabetics"),
    }
}

fn chr(o: u8) -> char {
    (o + b'a') as char
}

fn next(c: char) -> char {
    const NUM_OF_ALPHABETS: u8 = 26;
    match c {
        o if !o.is_alphabetic() => c,
        _ => chr(NUM_OF_ALPHABETS - 1 - ord(c)),
    }
}

fn convert<'a>(s: &'a str) -> impl Iterator<Item = char> + 'a {
    s.chars().filter(|c: &char| c.is_alphanumeric()).map(next)
}

pub fn encode(plain: &str) -> String {
    convert(plain)
        .enumerate()
        .flat_map(|(i, c): (usize, char)| {
            std::iter::once(' ')
                .filter(move |_: &char| i > 0 && i % 5 == 0)
                .chain(std::iter::once(c))
        })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    convert(cipher).collect()
}
