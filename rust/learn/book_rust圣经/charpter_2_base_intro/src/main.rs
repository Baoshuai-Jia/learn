mod var_main;

// RUST程序的入口函数,
fn main() {
    // 使用let来声明变量, 进行绑定, a是不可变的
    // 此处没有声明a的类型,编译器会自动推导a的类型: i32 ,有符号32位整数
    // 语句末尾以分号结尾
    let a = 10;
    // 主动指定b的类型为 i32
    let b: i32 = 20;
    // 这里有两点需要记忆的:
    // 1. 可以在数字30上带上类型i32, 表示: c 是 i32类型
    // 2. c 是可变的, mut是mutable的意思
    let mut c = 30i32;
    c += 1;
    //还可以在类型和数字之间添加下划线, 可读性更好
    let d = 30_i32;
    // 和其他函数一样, 可以使用一个函数的返回值作为另一个函数的参数
    let e = add(add(a, b), add(c, d));
    // println! 是宏调用,它看起来像函数, 但是它返回的是宏定义的代码块
    // 该函数的格式化输出的字符串输出到标准输出中(控制台中)
    // {} 是占位符, 在具体执行过程中, 会把e带入进去
    // Rust 使用 {} 来作为格式化输出占位符，其它语言可能使用的是 %s，%d，%p 等，由于 println! 会自动推导出具体的类型，因此无需手动指定
    println!("( a + b ) + ( c + d ) = {} ", e);
}
fn add(a: i32, b: i32) -> i32 {
    // 返回相加值, 这里可以省略return
    // 如果 a + b 后面添加`;`, 那么返回值就是`()`而不是i32
    a + b
}
