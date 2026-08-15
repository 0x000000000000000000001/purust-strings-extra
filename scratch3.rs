use fancy_regex::Regex;

fn main() {
    let re = Regex::new("a").unwrap();
    let parts: Vec<_> = re.split("babcb").map(|x| x.unwrap()).collect();
    println!("{:?}", parts);
}
