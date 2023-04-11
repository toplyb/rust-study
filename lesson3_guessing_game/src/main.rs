// 引入输入输出的功能
use std::io;
// 生成随机数
use rand::Rng;
//
use std::cmp::Ordering;

fn main() {
    // 输出
    println!("猜数字游戏开始");

    // 生成 1 - 100 之间的数字
    let rand_number = rand::thread_rng().gen_range(1..101);

    // 使用循环多次猜测
    loop {
        println!("请输入你猜的数字:");
        // 变量默认是不可变的，mut 表示可变
        // 生成一个可变的空字符串
        let mut guess = String::new();

        // read_line 获取用户输入的内容，其中把 guess 放入意思是将用户输入的内容赋值给 guess，同时 read_line 并不会覆盖字符串中本身的内容
        // & 表示这个参数是一个引用
        io::stdin().read_line(&mut guess).expect("失败");

        // trim 是去除开头和结尾的字符串
        // parse 是将字符串转换成数字，因为可以转换成多种数字类型，所以需要手动指定类型，而在一开始的时候，已经给 guess 指定了 u32 的类型，所以就不需要再指定了
        let guess: u32 = match guess.trim().parse() {
            // 处理输错的情况，输入的不是数字的时候，重新输入
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("恭喜🎉猜对了");
                println!("生成的数字是：{}，你猜的数字是：{}", rand_number, guess);
                break;
            }
        }
    }
}