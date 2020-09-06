//Telling rust to add Debug trait
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };

    //:? indicates to print the Debug output
    println!("Rect is {:?}", rect);
}