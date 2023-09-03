fn main(){

  let x = String::from("hello");

  take_ownership(x);

  let x = 5;

  make_copy(x);
  
}


fn take_ownership(some_string: String){

    println!("{}", some_string);
}

fn make_copy(some_integer: i32){
    println!("{}", some_integer);
}