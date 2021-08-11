use std::io;

pub fn conversion() {
    println!("请选择要运行的程序，\n 1) 摄氏度（°C）转华氏度（°F）\n 2) 华氏度（°F）转摄氏度（°C）");
    let flag =loop {
        let mut flag = String::new();

        io::stdin()
            .read_line(&mut flag)
            .expect("Failed to read line");

        let flag: u32 = match flag.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("输入错误，请重新输入！");
                continue;
            }
        };

        if flag == 1 || flag == 2 {
            break flag;
        } else {
            println!("输入错误，请重新输入！");
            continue;
        }
    };

    println!("请输入要转换的温度数值，不要包含任何符号！");
    loop {

        let mut degree = String::new();

        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line");

        let degree: f64 = match degree.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("输入错误，请重新输入！");
                continue;
            }
        };

        if flag == 1 {
            to_fahrenheit(degree);
        } else {
            to_celsius(degree);
        }

        break;
    }
}

fn to_celsius(fahrenheit: f64) -> () {
    let celsius = (((fahrenheit - 32.0) / (9.0 / 5.0)) * 100.0).round() / 100.0;

    println!("你输入的温度是：{:.2} 华氏度（°F），转换为摄氏度为：{:.2} 摄氏度（°C）", fahrenheit, celsius);
}

fn to_fahrenheit(celsius: f64) -> () {
    let fahrenheit = ((32.0 + (celsius * 9.0 / 5.0)) * 100.0).round() / 100.0;

    println!("你输入的温度是：{:.2} 摄氏度（°C），转换为华氏度为：{:.2} 华氏度（°F）", celsius, fahrenheit);
}

