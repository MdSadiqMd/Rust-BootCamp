pub fn main() {
    /*  loop {
        println!("This runs forever to stop this apply `break` keyword and to restart the loop use `continue`")
    } */

    // Multiple Loops --> If there are multiple loops are there we should give the loops labels and terminate them accordingly
    //                --> If outer loop breaks innter loop will also break
    /* let loops = 'outer_loop: loop {
        if true {
            loop {
                break 'outer_loop 20; // This breaks outer loop
                // break; // This breaks inner loop
            }
        }
        break 10; // THis breaks outer loop
    };
    println!("{}", loops) */

    // For loop
    /* for i in 0..10{
        println!("{}",i)
    } */

    // While loop
    let mut num = 5;
    while num != 0 {
        println!("{num}");
        num = num - 1;
    }
}
