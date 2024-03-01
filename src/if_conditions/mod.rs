use std::cmp::Ordering;

#[allow(dead_code)]
pub fn cond() {
    let my_age = 18;

    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Not old enough to vote"),
        Ordering::Greater => println!("You can vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    }

    // let age2 = 8;

    // // TODO: This is fucking sickkkk
    // match age2 {
    //     1..=18 => println!("Important birthday"),
    //     21 | 50 => println!("Important birthday 2"),
    //     65..=i32::MAX => println!("Important birthday 3"),
    //     _ => println!("Not important"),
    // }

    // let my_age: i32 = 7;

    // let can_vote: &str = if my_age >= 18 { "can" } else { "cant" };

    // println!("You {} vote", can_vote)

    // if age <= 1 {
    //     println!("You are a baby");
    // } else if age == 21 {
    //     println!("You are an adult")
    // } else {
    //     println!("You are a child");
    // }
}
