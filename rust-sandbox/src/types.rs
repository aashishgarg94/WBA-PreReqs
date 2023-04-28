pub fn run(){
    let x = 1; // default is i32

    let y = 2.5; // default is f64

    // Add explicit type
    let z: i64 = 454545454545454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}