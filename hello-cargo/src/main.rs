

fn main() {
    let width = 50;
    let height = 30;

    let rect1 = (50,32);

    let area1 = area(width, height);

    println!("area is {}", area1);


    println!("area2 is {}", get_area_2(rect1));
}

fn area(w: i32, h: i32) -> i32 {
    w * h
}


fn get_area_2 (dimensions: (i32, i32))->i32{

    dimensions.0 * dimensions.1
}