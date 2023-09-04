fn main(){


    let my_string = String::from("hello world");


    for (index, item ) in my_string.iter() {

        println!("index {}, item {}", index, item);
    }

}