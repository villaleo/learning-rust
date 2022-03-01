fn opening(n: i32) {
    let day: &str = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "11th",
        12 => "12th",
        _ => "",
    };
    println!("On the {} day of Christmas\nMy true love sent to me", day);
}

fn item_received(n: i32, capital_first: bool) {
    let item: &str = match n {
        1 => {
            if capital_first {
                "A partridge in a pear tree"
            } else {
                "a partridge in a pear tree"
            }
        },
        2 => "Two turtle-doves",
        3 => "Three French hens",
        4 => "Four calling birds",
        5 => "Five golden rings (five golden rings)",
        6 => "Six geese a-laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a-milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a-leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => "",
    };
    println!("{}", item);
}

pub fn main() {
    for i in 1..=12 {
        opening(i);
        for j in (1..i + 1).rev() {
            if i != 1 && j == 1 {
                print!("And ");
                item_received(j, false);
                continue;
            }
            item_received(j, true);
        }
        if i == 12 {
            println!("And a partridge in a pear tree");
        } else {
            println!("");
        }
    }
}
