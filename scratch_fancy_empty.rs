use fancy_regex::Regex;

fn main() {
    let re = Regex::new(r"a?").unwrap();
    let matches: Vec<_> = re.find_iter("").map(|m| m.unwrap().as_str().to_string()).collect();
    println!("Matches: {:?}", matches);
}
