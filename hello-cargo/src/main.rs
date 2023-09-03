fn main() {
    let mut s1 = String::from("hello");


    let s2 = &mut s1;
    let s3 = &mut s1;

    println!("{s2} {s3}");
}
