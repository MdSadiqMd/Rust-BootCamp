fn main(){
    /*Rust is a Statically and Strongly typed Programming Language */
    /* *** Even if we not define the x as Integer the Compiler at run type see the value and define it as an Integer */
    let x=4;
    println!("{}",x);
    /*x=5; --> It throws an Error as every variable in rust is immutable but it can be made mutable if we use the mut keyword*/
    let mut y=5;
    println!("{}",y);
    y=6;
    println!("{}",y);
}