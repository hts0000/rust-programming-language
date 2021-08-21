use rand::Rng;
use std::cmp::Ordering; // cmp == compare, Ordering是一个枚举类型,成员为Less,Greater,Equal
use std::io; // Rng是一个trait,它定义了随机数生成器应实现的方法

fn main() {
    println!("Guess the number!");

    // rand包并非标准库,但是官方实现该代码包,rust中一个包称为crate(箱子?)
    // 可以在crates.io这个网站上找到当前所有开源库的最新信息
    // thread_rng()提供实际使用的随机数生成器,它位于当前执行线程的本地环境中,并从操作系统获取seed(随机数种子)
    // gen_range(1..101)指示生成随机数的范围,左包含右不包含
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    // 无限循环
    loop {
        println!("Please input your guess.");

        // let表示创建一个变量
        // mut表示该变量可变(mutable),如果不加mut则表示该变量不可变(immutable)
        // String::new(), ::语法表示new()是String类型的一个关联函数。关联函数是针对类型实现的
        let mut guess = String::new();
        // let mut guess = "aaaaaaaaaa".to_string();

        // read_line将获取的输入附加在传入的字符串后
        // read_line()会返回一个Result类型(枚举类型enums)
        // 这里的Result类型内部包含两个值Ok和Err,操作成功时枚举值为Ok,操作失败时枚举值为Err
        // 如果Result值为Err,expect()会使程序崩溃,并显示传入expect的信息
        // 如果Result值为Ok,expect()会读取Ok的值并原样返回,这里会返回读取的字节数
        let read_bytes = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guess.trim()去除guess首尾空白字符
        // parse()将guess解析为数字,数字的类型由guess:u32指定
        // let guess:u32又再次创建了一个变量guess,但是因为转换了类型为u32,因此rust将之前的值隐藏了,所以可以复用guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed: {}, bytes = {}", guess, read_bytes); // 输出会换行,说明guess还带有换行符

        // trim()可以去除string实例首尾的空白字符
        // println!("You guessed: {}, bytes = {}", guess.trim(), read_bytes); //You guessed: 88, bytes = 3

        println!("You guessed: {}, bytes = {}", guess, read_bytes); //You guessed: 88, bytes = 3

        // match为模式匹配
        // guess.cmp(&secret_number)返回guess和secret_number比较的结果
        // 比较结果为Ordering类型,匹配到哪个值就进入哪个分支
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
