use std::fmt;

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue)
    }
}

fn main() {
    println!("{}", Color { red: 128, green: 255, blue: 90 } );
    println!("{}", Color { red: 0, green: 3, blue: 254 } );
    println!("{}", Color { red: 0, green: 0, blue: 0 } );
}
