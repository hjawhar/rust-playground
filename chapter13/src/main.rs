// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     let only_borrows = || println!("From closure: {:?}", list);

//     println!("Before calling closure: {:?}", list);
//     only_borrows();
//     println!("After calling closure: {:?}", list);
// }

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    println!("-------------------");
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("-------------------");

    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);

    // println!("-------------------");
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }
    println!("-------------------");


    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
