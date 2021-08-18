use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    thread,
    time::Duration,
};

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
struct Cacher<F, K, V> {
    calculation: F,
    cache: HashMap<K, V>,
}

impl<F, K, V> Cacher<F, K, V>
where
    F: Fn(&K) -> V,
    K: Hash + Eq,
{
    fn new(calculation: F) -> Cacher<F, K, V> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &V {
        match self.cache.entry(arg) {
            Entry::Occupied(occupied) => occupied.into_mut(),
            Entry::Vacant(vacant) => {
                let value = (self.calculation)(vacant.key());
                vacant.insert(value)
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        *num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| *a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(*v2, 2);
    }

    #[test]
    fn value_can_differ_return_type_from_parameter_type() {
        let mut c = Cacher::new(|_a| String::from("hello"));

        let v = c.value(5);
        assert_eq!(*v, "hello");
    }
}
