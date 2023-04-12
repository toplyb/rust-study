// 函数命名采用蛇形命名
// 多个参数之间使用逗号隔开
// -> i32 指定返回值类型
fn get_number(n: i32) -> i32 {
    println!("参数是 {}",n);
    return 3;
}

fn main() {
    let x = get_number(5);
    println!("返回值是 {}", x);
}