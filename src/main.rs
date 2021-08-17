use std::io;
mod christmas_song;
mod employee_management;
mod evaluation;
mod fibonacci;
mod piglatinize;
mod temperature;
mod tarait_bounds_test;
mod lifetime_annotations_test;

fn main() {
    loop {
        println!("===================================\n1. 摄氏与华氏温度的相互转换\n2. 生成 n 阶斐波那契数列\n3. 打印圣诞颂歌的歌词\n4. 统计学：平均数、中位数、众数\n5. 将字符串转换为 Pig Latin\n6. 公司花名册\n7. 使用 trait bounds 来修复 largest 函数\n8. 使用 'static 来修复 longest 函数\n请输入功能数，或者其他任意数字退出。\n===================================");

        let mut feature_number = String::new();
        io::stdin()
            .read_line(&mut feature_number)
            .expect("Failed to read line");

        let func_num: i32 = match feature_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入错误，请重新输入！");
                continue;
            }
        };

        match func_num {
            1 => temperature::conversion(),
            2 => fibonacci::generate(),
            3 => christmas_song::print_lyrics(),
            4 => evaluation::statistics(),
            5 => piglatinize::transition(),
            6 => employee_management::roster(),
            7 => tarait_bounds_test::test(),
            8 => lifetime_annotations_test::test(),
            _ => {
                println!("感谢使用！");
                break;
            }
        };
    }
}
