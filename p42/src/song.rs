//custom type named SongIter implementing the iterator trait
use std::io::Write;
use std::net::TcpStream;
struct SongIter {
    current: usize,
    gifts: Vec<&'static str>,
}

impl SongIter {
    fn new() -> Self {
        let gifts = vec![
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
        Self { current: 0, gifts }
    }
}

impl Iterator for SongIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.gifts.len() {
            let gift = self.gifts[self.current];
            self.current += 1;
            Some(gift)
        } else {
            None
        }
    }
}

fn numbered_song_iter() -> impl Iterator<Item = String> {
    let mut iter = SongIter::new();
    let mut count = 1;

    std::iter::from_fn(move || {
        if let Some(gift) = iter.next() {
            let formatted_gift = format!("{:02}: {}", count, gift);
            count += 1;
            Some(formatted_gift)
        } else {
            None
        }
    })
}

fn duplicate_n_times<I>(mut iter: I, n: usize) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
{
    let mut count = 0;
    std::iter::from_fn(move || {
        iter.next().map(|item| {
            count += 1;
            if count == n {
                count = 0;
            }
            item
        })
    })
}

fn song_to_string(iter: I) -> String {
    let mut result = String::new();
    for item in iter {
        result.push_str(&item);
        result.push('\n');
    }
    result
}

fn song_to_file<I>(iter: I, filename: &str) -> std::io::Result<()>
where
    I: Iterator<Item = String>,
{
    let mut file = std::fs::File::create(filename)?;
    for item in iter {
        writeln!(file, "{}", item)?;
    }
    Ok(())
}

fn song_from_tcp(port: u16) -> std::io::Result<()> {};

fn song_to_tcp_stream<I>(iter: I, stream: &mut TcpStream) -> std::io::Result<()>
where
    I: Iterator<Item = String>,
{
    for item in iter {
        stream.write_all(item.as_bytes())?;
        stream.write_all(b"\n")?;
    }
    Ok(())
}

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

    for day in 0..12 {
        println!("On the {} day of Christmas", days[day]);
        println!("My true love gave to me");

        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                print!("And ");
            }
            println!("{}", gifts[gift]);
        }

        println!(); // Add a blank line between verses
    }
}
