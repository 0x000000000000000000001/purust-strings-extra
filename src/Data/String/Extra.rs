pub fn Data_String_Extra_levenshtein(a: String, b: String) -> i64 {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    let mut d = vec![vec![0; b_chars.len() + 1]; a_chars.len() + 1];
    
    for i in 0..=a_chars.len() {
        d[i][0] = i;
    }
    for j in 0..=b_chars.len() {
        d[0][j] = j;
    }
    
    for i in 1..=a_chars.len() {
        for j in 1..=b_chars.len() {
            let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
            d[i][j] = std::cmp::min(
                d[i-1][j] + 1,
                std::cmp::min(d[i][j-1] + 1, d[i-1][j-1] + cost)
            );
        }
    }
    
    d[a_chars.len()][b_chars.len()] as i64
}

pub fn Data_String_Extra_sorensenDiceCoefficient(a: String, b: String) -> f64 {
    if a == b {
        return 1.0;
    }
    
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    if a_chars.len() < 2 || b_chars.len() < 2 {
        return 0.0;
    }
    
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
    
    (2.0 * intersection as f64) / ((a_chars.len() - 1 + b_chars.len() - 1) as f64)
}
