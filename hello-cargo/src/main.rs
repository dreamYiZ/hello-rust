use std::io;

fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    // for mut item in arr.into_iter() {
    //     item = item * 2;
    // }
    let mut i: usize = 0;

    while i < arr.len() {
        arr[i] = arr[i] * 2;
        i += 1;
    }

    // for (index, item) in arr.iter().enumerate() {
    //     arr[index] = item * 2;
    // }

    println!("Please enter a index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index Enter was not a number");

    let element = arr[index];

    println!("The value of element at index {index} is {element}");
}
