// fn main() {
//     let say = String::from("Cat");

//     //cloning: creates a copy on the stack and heap memory
//     // print_out(say.clone());

//     //borrowing: changes reference
//     print_out(&say);
//     println!("Again: {}", say);
// }

// // String  //cloning
// // &String, // borrowing technique
// fn print_out(to_print: &String) {
//     println!("{}", to_print);
// }

//Mutable Reference
fn main() {
    let mut my_vec = vec![1, 2, 3];
    println!("{:?}", my_vec);
    add_to_vec(&mut my_vec);
    println!("{:?}", my_vec);
}

fn add_to_vec(a_vec: &mut Vec<i32>) {
    a_vec.push(4);
}

//Rules of borrowing
// - At any given time, you can have either:
//   -- One mutable reference or
//   -- Any number of immutable references
// - References must always be valid
