//To fina a word in sentances
fn main() {
    let term = "Sandeep";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture Sandeep feverishly turned--in search of what?
     It is the same with books.
     and Sandeep";
    for line in quote.lines(){  //.lines return entire line
        if line.contains(term) {
            println!("{}", line)
        }
    }
}

// the lines keyword in rust returns the entire line of the string