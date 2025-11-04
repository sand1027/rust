/*
& :- it refers the address of the variable to the another variable
* :- it refers the value of the variable to the another variable
*/

fn main(){
    // let a = 10;
    // let r = &a;
    // let b = a + *r;
    // println!("{}",b);
    let needle = 15;
    let haystack = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    for item in &haystack{
        if *item == needle{
            println!("{} found in haystack", needle);
        }
    }
}