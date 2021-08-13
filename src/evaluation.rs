use std::collections::HashMap;

pub fn statistics() {
    let mut numbers =[171, 66, 8, 8, 3, 1213, 18, 365];
    println!("AVERAGE: {}", average(&numbers).unwrap());
    println!("MEDIAN: {}", median(&mut numbers).unwrap());
    println!("MODE: {}", mode(&numbers).unwrap());
}

fn average(numbers: &[i32]) -> Option<f64> {
    let length = numbers.len();

    match length {
        len if len > 0 => Some(numbers.iter().sum::<i32>() as f64 / length as f64),
        _ => None,
    }
}

fn median(numbers: &mut [i32; 8]) -> Option<i32> {
    numbers.sort();
    let length  = numbers.len() / 2;

    match length {
        len if len > 0 => Some(numbers[length]),
        _ => None,
    }
}

fn mode(numbers: &[i32]) -> Option<i32> {
    let mut map = HashMap::new();

    for number in numbers {
        let count = map.entry(*number).or_insert(0);
        *count += 1;
    }

    map.iter()
        .find_map(|(key, val)| if val == map.values().max().unwrap() {
            Some(*key)
        } else {
            None
        })
}