fn is_factor(n: usize, factor: usize) -> bool {
    n % factor == 0
}

pub fn raindrops(n: usize) -> String {
    let mut result = String::new();

    if is_factor(n, 3) {
        result.push_str("Pling");
    }

    if is_factor(n, 5){
        result.push_str("Plang");
    }

    if is_factor(n, 7){
        result.push_str("Plong");
    }

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
