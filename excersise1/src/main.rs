// find the digit in haystack

fn main() {
    let needle = 4;
    let haystack = [1,3,4,5,6,7,8,9,10];
    for item in &haystack{
        if *item == needle{
            println!("found {}", needle);
        }
    }
}
