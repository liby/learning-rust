use std::io;

pub fn generate() {
    println!("请输入一个范围在 0 <= n <= 30 的正整数！");

    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) =>  num,
            Err(_) => {
                println!("输入错误，请重新输入！");
                continue;
            }
        };

        match n {
            0..=30 => {
                println!("F({}) 为 {}", n, fib(n));
                break;
            },
            _ => {
                println!("输入错误，请重新输入！");
                continue;
            }
        }
    };
}


fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let mut index = 2;
    let mut current = 1;
    let mut next = 2;

    while index < n {
        index = index + 1;
        let previous = current;
        current = next;
        next = previous + current;
    };

    return current;
}
