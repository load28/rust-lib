#[derive(Debug)]
pub struct User {
    name: String,
    age: u32,
}

pub fn new_vec() {
    let mut users: Vec<User> = Vec::new();
    users.push(User {
        name: String::from("John"),
        age: 30,
    });

    let b: Vec<User> = vec![
        User {
            name: String::from("John"),
            age: 30,
        },
        User {
            name: String::from("Jane"),
            age: 30,
        },
    ];

    let option_vec = b.get(2);
    let default_vec = User {
        name: String::from("None"),
        age: 0,
    };

    let _option_vec: &User = match option_vec {
        Some(x) => x,
        None => &default_vec,
    };

    for i in &b {
        println!("i: {:?}", i);
    }
}
