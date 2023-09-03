fn main(){


    let mut str = String::from("Hello");

    str.push_str(", World!");


    println!("{str}");

    let s1 = "hello";
    let s2 = "world!";

    println!("{s1} {s2}");

    let s3 = "hello";
    let s4 = s3.clone();

    println!("s3 == s4? {} {}", s3 == s4, ".");
}