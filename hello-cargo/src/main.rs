fn main(){


    let my_string = String::from("hello world");


    for (index, item ) in my_string.chars().enumerate() {

        println!("index {}, item {}", index, item);
    }

}