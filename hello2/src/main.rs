fn greet(){
    println!("Hello, world!");
    let kannada = "Namaskara";
    let english = "Hello";
    let hindi = "Namaskar";
    let regions = [kannada, english, hindi];
    for region in regions.iter(){
        println!("{}", region);
    }
}
#[derive(Debug)]
    enum Cereal{
        Barley,
        Millet,
        Rice,
    }

fn main() {
  let mut grains : Vec<Cereal> = vec![];
  grains.push(Cereal::Rice);
  drop(grains);
  println!("{:?}", grains);
}
