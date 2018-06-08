// import a library/external dependency
use std::mem;

fn main() {
    // Note that functions that do not return anything will implicitly return an empty tuple.
    // mutability();
    // tuples();
    // arrays();
    // strings();
}

fn mutability() {
    // variables are immutable (constant) by default. mut makes them mutatable.
    let mut x: u32 = 5;
    println!("{}", x);
    // this only works with mut variables
    x = 10;
    println!("{}", x);
}

fn tuples() {
    // tuple example. Can intermix types.
    let tupl = (1, "fifty");

    // print value
    println!("{}", tupl.1);
    // print debug (strings are surrounded by quotes ("))
    println!("{:?}", tupl.1);
    // debug the entire tuple
    println!("{:?}", tupl);
    // debug with pretty for the entire tuple
    println!("{:#?}", tupl);

    // tuples that are too long cannot be printed
    // let tooLong = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("{:?}", tooLong);
}

fn arrays() {
    // array example
    let arra: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", arra);
    // print array item
    println!("{:?}", arra[0]);
    // print array length
    println!("{:?}", arra.len());
    // print array memory size
    println!("{:?}", mem::size_of_val(&arra));

    // taking slices. takes items 2,3, and 4. The min index is inclusive, the max index is exclusive.
    let slic = &arra[1..4];
    println!("{:?}", slic);
}

fn strings() {
    // string literal ("mystring") is a slice of a string (&str).
    // strings are compound types of slices put together into a string.

    // strings can be treated like slices because of this.
    // to create a String with the type String, use this.
    let the_string = String::from("Hello ");
    println!("{:?}", the_string);

    // string concatenation example. Note the reference (&another_string).
    let another_string = String::from("World!");
    let combined_string = the_string + &another_string;
    println!("{}", combined_string);
}