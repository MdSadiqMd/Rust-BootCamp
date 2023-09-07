fn main(){
    /*Integers */
    /*Here we have two types Signed and Unsigned Integer */
    /*in represent Integers from 0 to 2^n-1 */
    /*un represent Integers from -2^n to 2^n-1 */
    let a:u128=4;
    println!("{}",a);

    /*Floating Values */
    /*Here we two only one type f32 and f64 */
    /*If we not assigned anything compiler will default take it as f64 */
    let b:f32=5.34;
    println!("{}",b);

    /*Booleans */
    /*Here we have only one type bool */
    let c:bool=true;
    println!("{}",c);

    /*Character */
    /*Here we only have one type char */
    let d:char='A';
    println!("{}",d);
}