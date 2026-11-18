# day_02

## Goal
 - Re-learning ownership and borrowing
 - Making some small text processor to solidify in/out and ownership handling

## Notes
 - scope is what allows us to "drop" variables to save space
 - copying x into y is considered a "move" for Strings as it pushes x out of scope.
 - Rust never automatically does "deep" copies, so all automatic copying should be cheap
 - Similarly if we re-assign a value to x, the "old x" is dropped
 - If we do want a deep copy, we use x.clone(). This method actually copies heap data instead of moving it.

 - ** Why does this not work for ints? You don't have to .clone an int! This is because an int is on the stack and we know its size and it implements Copy (some types don't like Strings), Strings are on the heap since we can't guess how big they are" **

 - there is also a trait called Copy, but probably don't worry about it till I need to remember it.

 - What this means in practice is if you pass a String into something, it goes out of scope. If you pass an int, it does not due to Copy implementations. HOWEVER you could clone into a function and then it stays in scope outside.

 - you cannot modify references default. they have to be mutable references. HOWEVER once you make a mutable ref, there can only ever be 1 of it. It must be dropped to have another reference to it. This is some concurrency/data race issue thats just a rustism.
 - you also cannot have immutable references and mutable references together aka:
    let r1 = &s
    let r2 = &mut s - this line has an issue
 - we can manipulate scopes here to work our way around these issues though (and thats just good code baby)

 - string slices are so nice. we can do something like:
 let s = String::from("hello you animals");
 let hello = &s[0..5]; and that would take from index 0 to right before index 5 giving us a slice to "hello"

 - you can also do just ..5 as .. assumes starting at 0.

 - finally: ownership controls who is responsible for a value, but borrowing gives temporary access.

## What I Learned

 - NLL (non-lexical lifetimes) allows us to actually have both immutable and mutable refs to something, as long as the refs are handled with lifetimes.
 for ex:
 let mut message = String::from("hello");

 let mut ref = &message;

 println!("{reference"});

 function(&mut message);

 println!("{message}");

   this would actually compile because our call to "reference" is done before the function call. This pushes it out of the lifetime and makes the rest valid.

 - There's something called Elision lifetimes that has to do with the explicit placement of memory and our ref to it but I'm gonna just...not think about it for now.
