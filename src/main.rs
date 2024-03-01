// use std::collections::HashMap;

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
    // Enums
    // enum Days {
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday,
    // }

    // impl Days {
    //     fn is_weekday(&self) -> bool {
    //         match self {
    //             Days::Saturday | Days::Sunday => false,
    //             _ => true,
    //         }
    //     }
    // }

    // Days::Monday.is_weekday();
    // Days::Saturday.is_weekday();

    // Once string is moved to str2, str1 is no longer valid
    // let str1 = String::from("Hello");

    // let str2 = str1.clone();

    // println!("{} {}", str1, str2);

    // Hashmap

    // let mut heroes = HashMap::new();

    // heroes.insert("Superman", "Clark Kent");
    // heroes.insert("Batman", "Bruce Wayne");
    // heroes.insert("The Flash", "Barry Allen");

    // for (k, v) in heroes.iter() {
    //     println!("{}: {}", k, v);
    // }
}
