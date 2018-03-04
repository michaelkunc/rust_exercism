pub fn raindrops(n: usize) -> String {
    let mut result = String::new();
    match n {
        3 => result.push_str("Pling"),
        5 => result.push_str("Plang"),
        7 => result.push_str("Plong"),
        _ => result.push_str(&n.to_string()),
    }
    result
}
