use std::io::BufRead;
use std::{collections::HashMap, io};

pub fn roster() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    println!("请输入 Add <name> to <department> 将员工添加到指定部门");
    println!("请输入 List <department> 列出指定部门的所有员工");
    println!("请输入 All 获取所有部门的员工");
    println!("请输入 Quit 退出");

    for line in io::stdin().lock().lines() {
        let input = line.expect("请重新输入！");
        match Employee::edit(&input) {
            Some(Employee::Add { name, department }) => {
                employees.entry(department).or_default().push(name);
            }
            Some(Employee::List(department)) => match employees.get(&department) {
                Some(names) => {
                    for name in names {
                        println!("{}: {}", department, name);
                    }
                }
                None => println!("找不到这个部门!"),
            },
            Some(Employee::All) => {
                for (department, names) in &employees {
                    let mut names = names.clone();
                    names.sort();
                    for name in names {
                        println!("{}: {}", department, name);
                    }
                }
            }
            Some(Employee::Quit) => break,
            None => println!("输入错误！"),
        }
    }
}

enum Employee {
    Add { name: String, department: String },
    List(String),
    All,
    Quit,
}

impl Employee {
    fn edit(string: &str) -> Option<Self> {
        let words: Vec<&str> = string.trim().split_whitespace().collect();

        match words.as_slice() {
            ["All"] => Some(Employee::All),
            ["List", department] => Some(Employee::List(department.to_string())),
            ["Add", name, "to", department] => Some(Employee::Add {
                name: name.to_string(),
                department: department.to_string(),
            }),
            ["Quit"] => Some(Employee::Quit),
            _ => None,
        }
    }
}
