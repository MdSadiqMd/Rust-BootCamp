/* Exercise 3: Generic Functions with Trait Bounds
Write a function that takes two items implementing a Comparable trait and returns the "larger" one based on a compare_to method you define. */

trait Comparable {
    fn compare_to(&self, other: &Self) -> bool;
}

#[derive(Clone)]
struct Constants {
    value: i32,
}

impl Comparable for Constants {
    fn compare_to(&self, other: &Self) -> bool {
        self.value > other.value
    }
}

fn larger<'a, T: Comparable + Clone>(a: &'a T, b: &'a T) -> &'a T {
    // 'a will return the reference value, this will decrease the overhead for us to clone()
    if a.compare_to(b) {
        return a;
    } else {
        return b;
    }
}

pub fn main() {
    let first = Constants { value: 5 };
    let second = Constants { value: 10 };
    let larger_num = larger(&first, &second);
    println!("{}", larger_num.value);
}
