mod arrays;

use arrays::array_fn;

// fn main() {
//     println!("What is your name?");

//     let mut name: String = String::new();
//     let greeting: &str = "You are a poes!";

//     io::stdin()
//         .read_line(&mut name)
//         .expect("Didnt receive input");

//     println!("Hello {}! {}", name.trim_end(), greeting);
// }

// fn main() {
//     const ONE_MIL: u32 = 1_000_000;

//     let age: &str = "25";

//     let mut age: u32 = age.trim().parse().expect("Age wasnt assigned a number");

//     age = age + 1;

//     println!("Age: {}, I want ${}", age, ONE_MIL);
// }

// fn main() {
//     let random_num = rand::thread_rng().gen_range(1..101);

//     println!("Random : {}", random_num)
// }

fn main() {
    array_fn();
}
