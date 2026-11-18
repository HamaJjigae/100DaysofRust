fn main()
{
    let mut message = String::from("server started");

    println!("Length: {}", length_calc(&message));

    add_pre(&mut message, "[INFO] ");
    // we pass a mutable ref here cuz it has to be mutable on both ends
    println!("After pre-addition: {}", (message));
    // DOES NOT HAVE TO BE A REF. Some functions take variables automatically as refs like print.
    // kinda interesting.
    transfer_ownership(&message);
    println!("{}", (message));

    let wordtester = String::from("hello world");
    let word = first_word(&message);

    println!("{word}");
}

fn length_calc(message: &str) -> usize
{
    let length = message.len();
    length
}
// above is very easy taking a &str and giving back our len calculated by .len() which works on strs
// and Strings. It returns usize so our return needs to be set as usize.
fn add_pre(message: &mut String, pre: &str)
{
    message.insert_str(0, pre);
}
// above takes mutable ref and inserts to it (doable cuz its a mutable String, not a mutable str)
// use insert_str (index, str). we can also use insert(indx, char) but that would need to iterate
// through the chars of a string.
fn transfer_ownership(message: &str)
{
    dbg!(message);
    println!("ownership transferred");
}
// very simple again, this takes a non-referenced String, which means we now own it within this
// function. It will go out of scope in the main after this call.

fn first_word(wordtest: &str) -> &str
{
    let bytes = wordtest.as_bytes();

    for (i, &character) in bytes.iter().enumerate()
    {
        if character == b' '
        {
            let slice = &wordtest[..i];
            return slice;
        }
    }
    wordtest
}
