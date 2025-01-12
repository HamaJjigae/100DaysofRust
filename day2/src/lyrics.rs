use ordinal::Ordinal;

pub fn lyrics() {
    let words = [
        "And a partridge in a pear tree.\n",
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
    let mut counter = 0;

    //this is a very ugly way to handle the first iteration but it doesnt have uncessary checks
    println!(
        "On the {} day of Christmas my true love gave to me: ",
        Ordinal(counter + 1).to_string()
    );
    println!("A partridge in a pear tree.\n");
    counter += 1;

    while counter != 12 {
        println!(
            "On the {} day of Christmas my true love gave to me: ",
            Ordinal(counter + 1).to_string()
        );
        for i in (0..=counter).rev() {
            println!("{}", words[i]);
        }

        counter += 1;
    }
}
