enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!(
            "width: {}, height: {}, depth: {}",
            self.width, self.height, self.depth
        );
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(color: Color, weight: f64, dimensions: Dimensions) -> Self {
        Self {
            color,
            weight,
            dimensions,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 10.0,
        height: 5.0,
        depth: 2.0,
    };
    let small_box = ShippingBox::new(Color::Brown, 1.5, small_dimensions);
    small_box.print();
}
