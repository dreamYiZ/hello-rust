fn main() {
    // let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3];


    // println!("{}", slice.len());
    // assert_eq!(slice, &[2,3]);

    let s = String::from ("Hello World");


    // let s1 = &s[0..5];

    // let s2 = &s[6..s.len()];

    // println!("{}", s1);
    // println!("{}", s2);


    let s1 = &s[0..2];

    println!("{}",s1);
    let s1 = &s[..2];
    println!("{}",s1);

}
