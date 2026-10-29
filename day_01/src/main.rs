fn main()
{
    let mut stringy = String::new();
    // local mutable string
    let mut string2 = String::from("HI THERE");
    // local mutable string started as "HI THERE"
    let mut string3 = "HELLO".to_string();
    //local mutable string started from "HELLO". maybe adds overhead? idk why but i like this less
    let mut string4 = "xyz";
    // makes a string slice. slice here means we can change what it is but we cant modify it. AKA
    // cant add to or remove from it. so no .push_str("test")
    let mut number2: u16 = 8;
    // local set at spawn
    let number3: u16 = 8;
    //local immutable set at spawn
    const CONST_VALUE: u8 = 8;
    // compile-time constant
    static GLOBAL_VALUE: u8 = 8;
    // static variable

    // notes on u and i. its bits and i can't memorize the values that well. effectively i cuts the
    // range in half. u8 is 0-255, but i8 is -128-127. in general...start small and go up as the
    // compiler yells at you...

    //now lets talk const vs let. const is compile time so if we want it to be in a 'call' it has to
    //be defined in const fn so that its run at compile time. this is really nice for performance
    //instensive things that we know will never change. like we cant do
    //const = fn. but we can do let = fn (IGNORE MY SYNTAX PLEASE). Also note that const doesnt have
    //a guaranteed mem location. Its value can be subbed whenever its used.
    //static on the other hand has a guaranteed mem space. that means while theyre typically
    //immutable, they can contain things that are mutable like mutexes? atomics? idk what that is
    //in regards to anything. What this means tho is that consts are much more restrictive in what
    //you can do with them, but theyre also more memory free. Statics on the other hand have more
    //high-level options especially as it relates to mutability, but theyre stuck in a specific
    //memory space, which introduces memory integration (possibly?).
    //
    //
    //also heres some term reminders (its been a minute):
    //declaration: introducting a variable or term
    //initialization: giving it a value (intial value)
    //assignment: giving it a new value (mut)
    //binding: connecting a name with a value (god i LOVE derrida)

    let stringy1 = String::from("testtest");
    let (output, length) = upper_message(&stringy1);
    println!("{output} and {length}");
}
fn create_message() -> (String, usize)
{
    let mut tester = String::new();
    tester.push_str("test");

    let output = tester.len();

    return (tester, output);
}

// atomics and mutexes notes. I KNEW THIS WAS SOME MULTITHREAD SHIT. AtomicUsize allows operations
// to safely exist even if theres multiple threads. Mutex<T> (for example) is a sync mech as well
// (god i cant wait to relearn tokio) it allows threads to coordinate access so you don't get
// deadlocks.
//

fn upper_message(input: &str) -> (String, usize)
{
    let output = input.to_uppercase();
    let length = output.len();

    (output, length)
}

//some stuff i learned here (have to put this in readme as well). because of the Some/None enum we
//are stuck handling everything safely (its good i guess), so we need to if let Some(x) to properly
//check &str.chars().nth(x) which ensures it exists before we operate on it. also due to scope
//need to declare our variable outside the if let. we could also do a for loop but thats way less
//efficient. or we could turn our string into a vec and then access the index that way but thats
//overhead. tldr its not like python where we can just index a string. we need to convert to chars
//and check safely.
