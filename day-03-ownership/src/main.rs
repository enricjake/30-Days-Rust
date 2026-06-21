fn main() {
    let s1 = String::from("Rust");

    // s1 is moved to s2. s1 is no longer valid!
    let s2 = s1; 

    println!("s2 says: {}", s2);

    // TRY UNCOMMENTING THE LINE BELOW:
    // println!("s1 says: {}", s1);
}