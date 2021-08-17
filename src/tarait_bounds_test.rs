pub fn test() {
    let number_list = [34, 50, 25, 100, 65];
    println!("The largest number is {} by largest_with_clone", largest_with_clone(&number_list));
    println!("The largest number is {} by largest_with_ref", largest_with_ref(&number_list));
}

// If we don’t want to restrict the largest function to the types that implement the Copy trait, we could specify that T has the trait bound Clone instead of Copy. Then we could clone each value in the slice when we want the largest function to have ownership. Using the clone function means we’re potentially making more heap allocations in the case of types that own heap data like String, and heap allocations can be slow if we’re working with large amounts of data.

// https://www.reddit.com/r/learnrust/comments/84fyee/clone_and_copy_traits/dvpay90?utm_source=share&utm_medium=web2x&context=3
fn largest_with_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();

    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }

    largest
}

// Another way we could implement largest is for the function to return a reference to a T value in the slice. If we change the return type to &T instead of T, thereby changing the body of the function to return a reference, we wouldn’t need the Clone or Copy trait bounds and we could avoid heap allocations. Try implementing these alternate solutions on your own!

// https://www.reddit.com/r/learnrust/comments/84fyee/clone_and_copy_traits/dvpb69b?utm_source=share&utm_medium=web2x&context=3
fn largest_with_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    largest
}