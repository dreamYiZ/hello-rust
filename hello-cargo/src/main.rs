fn main(){

    // a variable only has one ownership

    let x1 = give_ownership();

    let x2 = String::from("hello");


    println!("x2 {}", x2);
    let x3 = take_and_give_ownership(x2.clone());

    println!("x2 {}", x2);


    println!("{x1}{x3}");
}


fn give_ownership()->String{
    let x = String::from("Yours");

    x
}


fn take_and_give_ownership(x: String)->String{
    x
}