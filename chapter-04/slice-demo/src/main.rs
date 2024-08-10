
fn main() {
    let s1 = String::from("Hello world");
    let first_word_s1 = first_word(&s1);
    let s2 = "Helo world";
    let first_word_s2 = first_word(&s2);

    println!("{}", first_word_s1);
    println!("{}", first_word_s2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}