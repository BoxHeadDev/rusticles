# The Twelve Days of Christmas Challenge

## Key Features

- Uses arrays to store days and gifts, avoiding redundant code.
- Loops efficiently iterate through the song structure.
- Handles the "and" in "Two turtle doves, and" correctly, ensuring the lyrics remain accurate.
- Reverse iteration (rev()) ensures that each day's gifts are printed in the correct order.


fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
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

    for (day_index, day) in days.iter().enumerate() {
        println!("\nOn the {day} day of Christmas, my true love sent to me:");

        for gift in (0..=day_index).rev() {
            println!("{}", gifts[gift]);
        }
    }
}
