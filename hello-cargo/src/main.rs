
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user = User {
        active: true,
        username: String::from("rentoo"),
        email: String::from("rentoo@ren.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("rentoo2"),
                                    String::from("rentoo2@rentoo.com"));


    println!("{:?}", user2);
// 
    let user3 = User{
        email: String::from("rentoo3@rentoo.com"),
        ..user2
    };

    println!("{:?}", user);
    println!("{:?}", user3);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
