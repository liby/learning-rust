use std::io;
pub mod temperature;
pub mod fibonacci;
pub mod christmas_song;

fn main() {
    loop {
        println!("===================================\n1. 摄氏与华氏温度的相互转换\n2. 生成 n 阶斐波那契数列\n3. 打印圣诞颂歌的歌词\n请输入功能数，或者其他任意数字退出。\n===================================");

        let mut feature_number = String::new();
        io::stdin().read_line(&mut feature_number).expect("Failed to read line");

        let func_num: i32 = match feature_number.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("输入错误，请重新输入！");
                continue;
            }
        };

        match func_num {
            1 => temperature::conversion(),
            2 => fibonacci::generate(),
            3 => christmas_song::print_lyrics(),
            _ => {
                println!("感谢使用！");
                break;
            }
        };
    }
}
