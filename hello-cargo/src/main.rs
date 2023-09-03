fn main() {
    let s1 = String::from("hello");

    let (s2,l) = calculate_length(s1);

    // println!("{s1}");

    println!("{s2} {l}");
}

fn calculate_length(s: String) -> (String, usize) {
    let s_len = s.len();

    (s, s_len)
}


