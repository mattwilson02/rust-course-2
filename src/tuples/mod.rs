pub fn tuples_fn() {
    let mut st1 = String::new();

    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }
}
