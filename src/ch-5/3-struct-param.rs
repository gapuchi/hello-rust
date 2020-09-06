struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };
    
    println!("The area of the rectangle is {}.", area(&rect));
}

//We do not want to take ownership so main can keep using it.
fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}