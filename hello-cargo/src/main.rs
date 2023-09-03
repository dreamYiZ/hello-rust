fn main(){

    let s1 = String::from("hello");

    let l = calculate_len(&s1);

    println!("{s1} {l}");

}


fn calculate_len(s: &String)->usize{
    s.len()
}