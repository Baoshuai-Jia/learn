fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    let s2 = &mut s;
    let s1 = &mut s;
    println!("{} {}", s1, s2);
}
fn change(s: &mut String){
    s.push_str(", world");
}
