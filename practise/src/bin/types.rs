/*
-> = It is the return type
Syntax :-
fn sum(a : i32, b:i32) -> i32{
a + b
}
NOTE :- we will not use return keyword in rust and no ; when we use -> as well

=> = It is the used in matches to compare
Syntax :-
match expression {
    pattern1 => expression,
    pattern2 => expression,
    pattern3 => expression,
}
match is nothing but switch
example :-
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
NOTE :- we will not use break keyword in rust and no ; when we use => as well
Match types :-
10 => :- it matches the exact number
10..=20 => :- it matches the range of numbers
_ => :- it matches anything
10|20 => :- it matches the number 10 or 20
*/


// to check if its even and return boolean value
fn is_even(n : i32) -> bool{
    n%2 == 0
}

fn main() {
    let a = 10;
    // let des1 = if is_even(a){
    //     "even"
    // }else{
    //     "odd"
    // }

    let des2 = match is_even(a){
        true => "even",
        false => "odd"
    };
    println!("{} is {}", a, des2);
}
