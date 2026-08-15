use fancy_regex::Regex;

fn main() {
    let re = Regex::new("(?i)abc").unwrap();
    let is_match = re.is_match("ABC").unwrap();
    println!("{}", is_match);
}
