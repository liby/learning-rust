// Try designing more experiments that vary the values and lifetimes of the references passed in to the longest function and how the returned reference is used. Make hypotheses about whether or not your experiments will pass the borrow checker before you compile; then check to see if you’re right!

pub fn test() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = "longer string is longer than long string";
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);
}

// https://stackoverflow.com/a/55159576/12119348
// https://www.reddit.com/r/learnrust/comments/cs1k8l/need_help_understanding_lifetimes/exem664?utm_source=share&utm_medium=web2x&context=3
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
