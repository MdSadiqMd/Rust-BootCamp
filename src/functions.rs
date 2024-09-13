pub fn main() {
    let a: u32 = 5;
    let b: u32 = 6;
    let sum: u64 = sum(a, b);
    println!("{}", sum)
}

fn sum(a: u32, b: u32) -> u64 {
    return (a + b) as u64;
}
