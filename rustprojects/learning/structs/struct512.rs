struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //Creating a new user instance using some of the values from user1
    // this is a explicit way to do it. There is a more simple way
    //let user2 = User {
    //    email: String::from("another@example.com"),
    //    username: String::from("anotherusername567"),
    //    active: user1.active,
    //    sign_in_count: user1.sign_in_count,
    //};
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    //In this way, we are saying that he can pick the rest of the values from user1 instance
}
