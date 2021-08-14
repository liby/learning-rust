use std::collections::HashMap;

pub fn statistics() {
    let numbers = vec![171, 66, 8, 8, 3, 1213, 18, 365, 1, 1, 1, 2, 2, 2];
    println!("AVERAGE: {}", average(&numbers).unwrap());
    println!("MEDIAN: {}", median(&numbers).unwrap());
    println!("MODE: {:#?}", mode(&numbers));
}

fn average(numbers: &Vec<i32>) -> Option<f64> {
    let length = numbers.len();

    match length {
        len if len > 0 => Some(numbers.iter().sum::<i32>() as f64 / length as f64),
        _ => None,
    }
}

fn median(numbers: &Vec<i32>) -> Option<f64> {
    let mut numbers = numbers.clone();
    numbers.sort();

    let mut length = (numbers.len() + 1) / 2;
    if numbers.len() % 2 == 0 {
        length = numbers.len() / 2;
    }

    match length {
        len if len > 0 => Some((numbers[len] + numbers[len - 1]) as f64 / 2.0),
        _ => None,
    }
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, value)| value == max_value)
        .map(|(&key, _)| key)
        .collect()
}
