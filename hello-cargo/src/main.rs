fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count %2 == 0 {
            println!("odd {}", count);

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 15 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}