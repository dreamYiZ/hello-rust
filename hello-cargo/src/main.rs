fn main(){


    let mut v = Vec::new();


    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);

    println!("{:?}", v);



    let v2 = vec![1,2,3,4,5,6];

    let third : &i32 = &v[2];


    println!("third is {}", third);

    let third2 :Option<&i32> = v.get(2);


    match third2 {
        Some(third2) => println!("the third is {}", third2),
        None =>  println!("there is no third2")
    }
}