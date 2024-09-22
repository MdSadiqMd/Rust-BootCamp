pub fn main() {
    // Slice is a refernce for a part of string that we hold by which ohter part of program will not mutate it

    // 1. Basic slice from an array
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4]; // Slice from index 1 to 3
    println!("Original array: {:?}", arr);
    println!("Sliced array (arr[1..4]): {:?}", slice);

    // 2. Mutating a slice
    let mut mut_arr = [100, 200, 300, 400, 500];
    let mut_slice = &mut mut_arr[2..4]; // Mutable slice from index 2 to 3
    println!("Before mutation: {:?}", mut_slice);
    mut_slice[0] = 999; // Changing value at index 2 in the original array
    println!("After mutation: {:?}", mut_slice);
    println!("Updated array: {:?}", mut_arr);

    // 3. Slicing a vector
    let vec = vec![1, 2, 3, 4, 5, 6];
    let vec_slice = &vec[0..3]; // Slice from index 0 to 2
    println!("Vector: {:?}", vec);
    println!("Sliced vector (vec[0..3]): {:?}", vec_slice);

    // 4. Passing a slice to a function
    let data = [5, 10, 15, 20, 25];
    print_slice(&data[1..4]); // Passing a slice of array

    // 5. Slicing a string
    let s = String::from("Hello, Rust!");
    let slice_str = &s[0..5]; // Slicing string
    println!("Original String: {}", s);
    println!("Sliced String: {}", slice_str);

    // 6. Length of slices
    let num_slice = &[1, 2, 3, 4, 5];
    println!("Length of the slice: {}", num_slice.len());
}

// A function that takes a slice as a parameter
fn print_slice(slice: &[i32]) {
    println!("Slice passed to function: {:?}", slice);
}
