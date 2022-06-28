fn main() {
    let s = String::from("Hello"); //immutable
    let mut m = String::from("Hello"); // mutable
    m.push_str(" world");

    println!("{}", m);

    let s_copy = s.clone();

    println!("{}", s_copy);

    let abc = String::from("abcdeghijklmnopqrstuvwxyz");
    let (s, length) = calculate_length(&abc);
    println!("{}", s);

    // Testing end of chapter 1  
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    println!("{}", s1);
    let s2 = String::from("hello"); // s2 comes into scope
    println!("{}", s2);

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
    println!("{}", s3);

    // println!("{} {} {}", s1, s2, s3)

    // let mut a = String::from("Hakuna");
    // let b = String::from("Matataa");
    // let final_word = concatenate_word(a,b);
    // println!("{}", final_word);
}

fn calculate_length(s: &String) -> (&String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

// fn concatenate_word(&mut String a) -> &mut String {
//     a.push_str(String::from("Hello"));
//     &mut a
// }