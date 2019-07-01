fn main() {
    let count = ["first", "second", "third",
                "fourth", "fifth", "sixth",
                "seventh", "eighth", "ninth",
                "tenth", "eleventh", "twelfth"];
    let days = ["A partridge in a pear tree",
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
                "Twelve drummers drumming"];
    for i in 0..12 {
        println!("\nOn the {} day of Christmas my true love sent to me", count[i]);
        for j in (0..i+1).rev() {
            println!("{}", days[j])
        }
    }
}
