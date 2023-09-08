fn main() {
    // Defining and Instantiating Structs
    defining_and_instantiating_structs();

    // Using the Field Init Shorthand
    using_field_init_shorthand();
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

// Example: Using the Field Init Shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Field names are the same as struct attribute names
// So, shorthand notation is allowed
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
fn using_field_init_shorthand() {
    println!("using_field_init_shorthand():");
    let user1 = build_user(
        "barney.rubble@slate-rock-n-gravel.com".to_string(),
        "brubble".to_string(),
    );
    println!("`user1` is: {:#?}", user1);
    let user2 = build_user_shorthand(
        "fred.flintstone@slate-rock-n-gravel.com".to_string(),
        "fflintstone".to_string(),
    );
    println!("`user2` is: {:#?}", user2);
}


