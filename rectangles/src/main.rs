fn main() {
    let (width, length) = (30, 50);
    let a = area1(width, length);
    let b = area2((width, length));
    let rect = Rectangle { width: 30, length: 50 };
    let c = area3(&rect);
    let d = rect.area();
    println!("Area = width * length = {} * {} = {}", width, length, a);
    println!("Area using tuple: {}", b);
    println!("Area using struct: {}\n{:#?}", c, rect); // #? = pretty print
    println!("Area using method: {}", d);
    let rect1 = Rectangle { width: 10, length: 40 };
    let rect2 = Rectangle { width: 60, length: 45 };
    println!("{}, {}", rect.can_hold(&rect1), rect.can_hold(&rect2));
    let rect3 = Rectangle::square(3);
    println!("{:#?} area = {}", rect3, rect3.area());
}

fn area1(w: u32, l: u32) -> u32 {
    w * l
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(r: &Rectangle) -> u32 {
    r.width * r.length
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.length <= self.length
    }
    
    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}
