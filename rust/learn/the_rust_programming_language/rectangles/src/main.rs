fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rectangle : {}", rectangle.area())
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn width(&self) ->bool{
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
