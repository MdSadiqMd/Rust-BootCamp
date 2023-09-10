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

    /*In the same scope we can create a new variable and assign a value for it */
    let z=7;
    println!("{}",z);
    let z=8;/*Will **Not throw error */
    println!("{}",z);

    /*Const --> The const variable should be in Upper Case
            --> It cannot be re-Written as let 
            --> Should specify the type when defining a const Variable*/
    const A:u32=9;
    println!("{}",A);
}