#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){

    let mut list = [
        Rectangle {width :10 ,height:6},
        Rectangle { width: 3, height: 2},
        Rectangle {width :7, height:5},
    ];

    list.sort_by_key(|r| r.width);

    println!("{:#?}" ,list);
}