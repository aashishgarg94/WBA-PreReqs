pub fn run(){
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');
    println!("{}", hello);

    // Push string
    hello.push_str("orld!");
    println!("{}", hello);

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}