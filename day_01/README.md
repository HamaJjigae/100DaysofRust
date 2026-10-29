# day_01

## Goal

 - Relearn simple syntax, simple borrowing, make a simple function with some kind of var handling and export.

## Notes

## What I Learned

 - const is compile time, so if u want it to be 'called' it needs to be within const fn (great for performance bad for compiling)
 - const has no guaranteed mem location so its value can be subbed wherever its used cheaply.
 - static has a guaranteed mem space, this means while its usually immutable it can carry mutable objects like mutexes or atomics
 - term rehash: declaration = introducing a variable or term. initialization = giving it an initial value. assignment = giving a new value. binding = connecting a name with a value (derrida).
 - atomics allow operations to exist safely with multiple threads
 - mutexes is a synch mech that allows threads to coordinate exclusive access to data.
 - if you wanna modify Strings by letter you need to do some conversions.
 so like...String -> .chars().nth(0) gives us the char at 0. but we need to also check for the Options<T> enum with Some/None for safety.
 so we could do Some(x) and then check &str.chars().nth(x) to get to x. (this extracts from the Option)
 - if we need to keep checking a string for indexes we should just convert it to a Vec (more memory).
 - .to_uppercase is a string return. but if u go from a char it becomes a ToUppercase iterator and needs to also be .to_string.
