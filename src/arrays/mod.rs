pub fn array_fn() {
    let arr: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut loop_idx = 0;

    // for val in arr.iter() {
    //     println!("Val: {}", val);
    // }

    while loop_idx < arr.len() {
        println!("Arr: {}", arr[loop_idx]);
        loop_idx += 1
    }

    // loop {
    //     if arr[loop_idx] % 2 == 0 {
    //         loop_idx += 1;

    //         continue;
    //     }

    //     if arr[loop_idx] == 9 {
    //         break;
    //     }
    //     println!("Val: {}", arr[loop_idx]);
    //     loop_idx += 1;
    // }
}
