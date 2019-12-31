pub fn reply(message: &str) -> &str {
    let bytes = message.trim().as_bytes();
    let is_empty = bytes.is_empty();
    let is_question = !is_empty && *bytes.last().unwrap() == '?' as u8;
    let is_yelling = !is_empty
        && bytes
            .iter()
            .filter(|x| (**x as char).is_alphabetic())
            .all(|x| (*x as char).is_uppercase())
        && bytes
            .iter()
            .filter(|x| (**x as char).is_alphabetic())
            .count()
            > 0;
    if is_question && is_yelling {
        "Calm down, I know what I'm doing!"
    } else if is_question {
        "Sure."
    } else if bytes.is_empty() {
        "Fine. Be that way!"
    } else if is_yelling {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
