//Create a function which prints the lyrics to the
//Christmas carol “The Twelve Days of Christmas”

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

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for (day_index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas", day);
        println!("My true love gave to me");

        for gift in (0..=day_index).rev() {
            if gift == 0 {
                print!("And ");
            }
            println!("{}", gifts[gift]);
        }

        println!(); // Add a blank line between verses
    }
}
