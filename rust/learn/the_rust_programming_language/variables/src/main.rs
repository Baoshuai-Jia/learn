fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is : {}", x);
    }
    println!("The value of x is: {}", x);
    let mut a: i8 = 127;
    println!("The value of a is :{}", a);
    // a = a+1;
    // println!("The value of a + 1 is :{}",a);
    // a = a +1;
    // println!("The value of a + 1 + 1 is :{}",a);
    a = 127_i8.wrapping_add(1);
    println!("The value of a + 1 + 1 + 1 is :{}", a);
    println!("The value of 127_i8 + 1 with checked methods is :{:?}",127_i8.checked_add(1));
    let (value,bool) = 127_i8.overflowing_add(1);
    println!("The result(value,bool) of 127_i8 + 1 with overflowing_add methods is :{:?} , {:?}",value,bool);
    println!("The value of 127_i8 + 1 with saturating is : {}",127_i8.saturating_add(1));

}