pub fn reply(message: &str) -> &str {
    let mut msg = message.to_string().trim().to_string();
    let mut alphabetic = false;
    let mut yell = false;
    let mut question = false;
    for alph in msg.chars() {
        if alph.is_alphabetic() {
            alphabetic = true;
            break;
        }
    }
    if msg.to_uppercase() == msg && alphabetic == true {
        yell = true;
    }
    if msg.ends_with("?") {
        question = true;
    }
    let mut result: &str = "";
    if msg.len() == 0 {
        result = "Fine. Be that way!";
    } else if yell == true && question == true {
        result = "Calm down, I know what I'm doing!";
    } else if yell == true && question == false {
        result = "Whoa, chill out!";
    } else if yell == false && question == true {
        result = "Sure.";
    } else {
        result = "Whatever.";
    }
    result
}
