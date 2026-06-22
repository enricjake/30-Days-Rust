fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); 
    println!("The length of '{}' is {}.", s1, len);

    // Array of 5 elements, all multiples of 5
    let numbers: [i32; 5] = [5, 10, 15, 20, 25];

    // We pass a reference to the array using '&' to borrow it
    divide_and_list(&numbers);

    // Because we only borrowed the array, it is still valid here!
    println!("Original array first element is still readable: {}", numbers[0]);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// This function borrows the array. It does not own it.
fn divide_and_list(arr: &[i32; 5]) {
    println!("--- Dividing array elements by 5 ---");
    
    // Iterating over the elements of the borrowed array
    for element in arr {
        let result = element / 5;
        println!("Original: {} / 5 = {}", element, result);
    }
}