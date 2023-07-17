trait Drawable {
    fn draw(&self);
}

struct Circle;
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

struct Square;
impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

fn main() {
    let circle = Circle;
    let square = Square;
    
    circle.draw(); // Gọi phương thức draw cho Circle
    square.draw(); // Gọi phương thức draw cho Square
}
