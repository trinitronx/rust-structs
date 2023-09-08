fn main() {
    // Defining and Instantiating Structs
    defining_and_instantiating_structs();
}

// Example: Defining and Instantiating Structs
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn defining_and_instantiating_structs() {
    println!("defining_and_instantiating_structs()");
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("`user1` is: {:#?}", user1);
    user1.email = String::from("anotheremail@example.com");
    println!(
        "`user1` fields: `active`: {}, `username`: {}, `email`: {}, `sign_in_count`: {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );
}
