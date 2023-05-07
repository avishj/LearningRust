fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninht",
        "tenth", "eleventh", "twelveth",
    ];
    const PASSAGES: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
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

    for (i, &day) in DAYS.iter().enumerate() {
        println!("On the {day} day of Christmas, my true love sent to me");
        for j in (0..=i).rev() {
            println!("{}", PASSAGES[j])
        }
        println!();
    }
}
