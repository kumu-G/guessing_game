use rand::Rng; // trait
use std::cmp::Ordering;
use std::io; // prelude // 枚举类型 三个变体（值）

fn main() {
    println!("猜数!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("神秘数字是：{}", secret_number);

    loop {
        println!("猜测一个数");

        // let mut foo = 1;
        // let bar = foo; // immutable

        // foo = 2;

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");
        // io::Result Ok Err

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入正确的数字");
                continue;
            }
        };
        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
