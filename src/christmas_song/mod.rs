pub fn print_lyrics() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for day in 1..13 {
        match day {
            1 => get_template("first"),
            2 => get_template("second"),
            3 => get_template("third"),
            4 => get_template("fourth"),
            5 => get_template("fifth"),
            6 => get_template("sixth"),
            7 => get_template("seventh"),
            8 => get_template("eighth"),
            9 => get_template("ninth"),
            10 => get_template("tenth"),
            11 => get_template("eleventh"),
            12 => get_template("twelfth"),
            _ => break,
        }

        for index in (0..day).rev() {
            println!("{}", gifts[index]);
        }
    }
}

fn get_template(day: &str) {
    println!("On the {} day of Christmas my true love sent to me", day);
}
