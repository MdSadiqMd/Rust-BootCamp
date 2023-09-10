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

    /*Tuples */
    /*Tuple is immutable in default but we can make them mutable using mut Keyword */
    let tup:(i32,bool,char)=(1,true,'s');
    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);

    /*Arrays */
    /*Arrays is immutable in default but we can make them mutable using mut Keyword */   
    let arr:[i32;5]=[1,2,3,4,5];
    println!("{}",arr[0]);
}