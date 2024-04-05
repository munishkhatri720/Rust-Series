fn main() {
    println!("Hello World !");
    let x = 5;
    let y : i128 = 1234;
    println!("x = {0} y = {1}" , x , y);
    xyz();
    mutualble_variable();
    rust_constants();
    conversion();
}    


fn xyz() {
    print!("Hello world a new line \n character");
}


fn mutualble_variable () {
    let mut x= 12;
    println!("x was {0}" , x);
    x = 5;
    println!("Now x is {0}" , x)
}


fn rust_constants () {
    const PI : f64 = 3.14;
    println!("Value of Pie is = {}" , PI);

}



fn conversion() {
    let x : f64 = 54.321 ; 
    let i = x as u16;
    println!("i = {0}" , i)
}




// comment using rust 


/* Mutli line comment using rust */