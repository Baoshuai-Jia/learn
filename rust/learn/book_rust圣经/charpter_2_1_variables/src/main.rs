fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;// error[E0384]: cannot assign twice to immutable variable `x`
    println!("The value of x is: {}", x);
}
