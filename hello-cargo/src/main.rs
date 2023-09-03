fn main(){


    let mut s = String::from("hello");

    let r2 = &s;
    let r3 = &s;

    let r4 = &mut s;

    println!("{r2}{r3}{r4}")
}