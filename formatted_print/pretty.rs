#[derive(Debug)]
struct Animal<'a> {
    name: &'a str,
    age: u16,
}

fn main() {
    let dog = Animal { name: "Cheese", age: 5 };
    println!("Our dog is: {dog:?}");
    println!("Our dog is: {dog:#?}");
}
