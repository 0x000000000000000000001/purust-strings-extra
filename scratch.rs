fn main() {
    let a = "ALGORITHMS ARE FUN";
    let b = "LOGARITHMS ARE NOT";
    
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    let mut a_bigrams = std::collections::HashMap::new();
    for i in 0..a_chars.len()-1 {
        *a_bigrams.entry((a_chars[i], a_chars[i+1])).or_insert(0) += 1;
    }
    
    let mut b_bigrams = std::collections::HashMap::new();
    for i in 0..b_chars.len()-1 {
        *b_bigrams.entry((b_chars[i], b_chars[i+1])).or_insert(0) += 1;
    }
    
    let mut intersection = 0;
    for (k, v) in a_bigrams {
        if let Some(v2) = b_bigrams.get(&k) {
            intersection += std::cmp::min(v, *v2);
        }
    }
    
    let coeff = (2.0 * intersection as f64) / ((a_chars.len() - 1 + b_chars.len() - 1) as f64);
    println!("Multiset: {}", coeff);
    
    let mut a_set = std::collections::HashSet::new();
    for i in 0..a_chars.len()-1 {
        a_set.insert((a_chars[i], a_chars[i+1]));
    }
    let mut b_set = std::collections::HashSet::new();
    for i in 0..b_chars.len()-1 {
        b_set.insert((b_chars[i], b_chars[i+1]));
    }
    let intersect_set = a_set.intersection(&b_set).count();
    let coeff_set = (2.0 * intersect_set as f64) / ((a_chars.len() - 1 + b_chars.len() - 1) as f64);
    println!("Set: {}", coeff_set);
}
