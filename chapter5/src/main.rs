fn main() {
    let user1 = create_user(
        String::from("UKF"),
        String::from("potato123"),
        String::from("hakuna@matata.com"),
        false,
        19,
    );

    println!(
        "user1: {:?} {} {}",
        user1,
        user1.is_active(),
        user1.is_adult()
    );
}

#[derive(Debug)]
struct User {
    username: String,
    password: String,
    email: String,
    is_active: bool,
    age: i8,
}

impl User {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn create_user(
    username: String,
    password: String,
    email: String,
    is_active: bool,
    age: i8,
) -> User {
    User {
        username,
        password,
        email,
        is_active,
        age,
    }
}
