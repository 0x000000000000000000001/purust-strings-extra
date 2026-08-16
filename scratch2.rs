fn sorensen_dice(a: &str, b: &str) -> f64 {
    if a == b { return 1.0; }
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    if a_chars.len() < 2 || b_chars.len() < 2 { return 0.0; }
    
    // Set approach
    let mut a_bigrams = std::collections::HashSet::new();
    for i in 0..a_chars.len()-1 {
        a_bigrams.insert((a_chars[i], a_chars[i+1]));
    }
    let mut b_bigrams = std::collections::HashSet::new();
    for i in 0..b_chars.len()-1 {
        b_bigrams.insert((b_chars[i], b_chars[i+1]));
    }
    
    let intersection = a_bigrams.intersection(&b_bigrams).count() as f64;
    (2.0 * intersection as f64) / ((a_chars.len() - 1 + b_chars.len() - 1) as f64)
}

fn normalize(n: f64) -> i64 {
    (n * 100000.0).round() as i64
}

fn check(a: &str, b: &str, expected: f64) {
    let actual = normalize(sorensen_dice(a, b));
    let exp = normalize(expected);
    if actual != exp {
        println!("FAIL: {} vs {}: expected {} (raw {}), got {} (raw {})", a, b, exp, expected, actual, sorensen_dice(a, b));
    }
}

fn main() {
    check("CONSERVATIONALISTS", "CONVERSATIONALISTS", 0.7647059);
    check("WHIRLED", "WORLD", 0.2000000);
    check("COMPLEMENT", "COMPLIMENT", 0.7777778);
    check("BAZAAR", "BIZARRE", 0.36363637);
    check("ACCESSARY", "ACCESSORY", 0.7500000);
    check("ALGORITHMS ARE FUN", "LOGARITHMS ARE NOT", 0.5882353);
    check("ASSISTANCE", "ASSISTANTS", 0.7777778);
    check("ALL TOGETHER", "ALTOGETHER", 0.8000000);
    check("IDENTICAL STRINGS", "IDENTICAL STRINGS", 1.0000000);
}
