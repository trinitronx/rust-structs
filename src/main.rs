fn main() {
    // Defining and Instantiating Structs
    defining_and_instantiating_structs();
    println!("\n");

    // Using the Field Init Shorthand
    using_field_init_shorthand();
    println!("\n");

    // Creating Instances from Other Instances with Struct Update Syntax
    struct_update_syntax();
    println!("\n");

    // Using Tuple Structs Without Named Fields to Create Different Types
    tuple_structs();
    println!("\n");

    // Unit-Like Structs Without Any Fields
    unit_like_structs();
    println!("\n");
}

// Example: Defining and Instantiating Structs
/// Example Struct for storing User data
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

// Example: Creating Instances from Other Instances with Struct Update Syntax
fn struct_update_syntax() {
    println!("struct_update_syntax():");
    let user1 = User {
        active: true,
        username: String::from("brubble"),
        email: String::from("barney.rubble@slate-rock-n-gravel.com"),
        sign_in_count: 1,
    };
    println!("`user1` is: {:#?}", user1);
    let user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("`user2` is: {:#?}", user2);
    println!("`user1` is still: {:#?}", user1); // No compile error b/c String fields were cloned

    // Shorthand Struct Update Syntax
    let user3 = User {
        email: String::from("struct-update@example.com"),
        ..user1 // = just use all other values from user1
                // FYI! b/c String does NOT implement Copy trait,
                // String fields from user1 are __moved__ into user3's fields
    };
    println!("`user3` is: {:#?}", user3);

    // Because user1's String fields were moved, the following won't compile
    // println!("`user1` is now unusable: {:#?}", user1); // Compile Error: borrow of partially moved value: `user1`
    // partial move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
}

// Example: Using Tuple Structs Without Named Fields to Create Different Types
/// A Tuple Struct for representing an RGB color,
/// composed of 3x i32 numbers
#[derive(Debug, Copy, Clone)]
struct Color(i32, i32, i32);
/// A Tuple Struct for representing an R3 Point,
/// composed of 3x i32 numbers for X, Y, and Z coords
#[derive(Debug)]
struct Point(i32, i32, i32);
/// Rust also supports structs that look similar to tuples, called tuple structs.
/// Tuple structs have the added meaning the struct name provides but don’t
/// have names associated with their fields;
/// rather, they just have the types of the fields.
///
/// When to use Tuple Structs
/// Tuple structs are useful when you want to give the whole tuple a name and
/// make the tuple a different type from other tuples, and when naming each
/// field as in a regular struct would be verbose or redundant.
fn tuple_structs() {
    println!("tuple_structs()");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("`black` is: {:?}", black);
    println!("`origin` is: {:?}", origin);

    let black2 = &mut Color(0, 0, 0);
    let mut black_plus_one = &black2;
    tuple_structs_are_unique_types(black2); // this works

    println!("`black`+1 is: {:?}", black2);

    // tuple_structs_are_unique_types(origin); // Compile Error: mismatched types
    // expected `Color`, found `Point`
}

/// Each struct you define is its own type,
/// even though the fields within the struct might have the same types.
/// For example, a function that takes a parameter of type Color
/// cannot take a Point as an argument,
/// even though both types are made up of three i32 values.
fn tuple_structs_are_unique_types(c: &mut Color) -> &mut Color {
    // tuple struct instances are similar to tuples
    // you can destructure them into their individual pieces,
    // and you can use a . followed by the index to access an individual value.
    c.0 += 1;
    c.1 += 1;
    c.2 += 1;
    c
}

// Example: Unit-Like Structs Without Any Fields
/// An example Unit-Like Struct called: "AlwaysEqual"
#[derive(Debug)]
struct AlwaysEqual;

/// You can also define structs that don’t have any fields!
/// These are called unit-like structs because they behave similarly to (),
/// the unit type
/// 
/// When to Use Unit-Like Structs
/// Unit-like structs can be useful when you need to implement a trait on some
/// type but don’t have any data that you want to store in the type itself.
fn unit_like_structs() {
    println!("unit_like_structs()");
    let subject = AlwaysEqual;
    println!("`subject` is: {:#?}", subject);
}