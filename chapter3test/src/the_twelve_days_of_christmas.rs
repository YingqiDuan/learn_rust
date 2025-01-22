pub(crate) fn main() {
    let days = [
        "first","second","third",
        "fourth","fifth","sixth",
        "seventh","eighth","ninth",
        "tenth","eleventh","twelfth",
    ];

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

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me:", days[day]);

        for gift in (0..=day).rev() {
            if day>0 && gift==0{
                print!("And ");
            }
            println!("{}", gifts[gift]);
        }
        println!();
    }
}