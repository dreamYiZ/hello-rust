fn main() {
    let mut s = String::from("foo");

    s.push_str(" bar");

    let s2 = "!";

    s.push_str(&s2);
    s.push_str(s2);

    println!("{}", s);
}
