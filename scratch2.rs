use fancy_regex::Regex;
use std::borrow::Cow;

fn main() {
    let re = Regex::new("a").unwrap();
    let rep = re.replace("abc", "d");
    let rep_all = re.replace_all("abcabc", "d");
    println!("{} {}", rep, rep_all);
}
