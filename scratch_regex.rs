use fancy_regex::Regex;

fn main() {
    let re = Regex::new(r"[\x{10000}-\x{10FFFF}]").unwrap();
    println!("Astral regex OK");
    let re2 = Regex::new(r"[\u2000-\u206f]").unwrap();
    println!("Normal uXXXX OK");
}
