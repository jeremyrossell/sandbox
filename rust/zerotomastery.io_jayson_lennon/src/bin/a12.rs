// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColour {
    Brown,
    Green,
}

struct ShippingBox {
    dimension_x: i32,
    dimension_y: i32,
    dimension_z: i32,
    weight: f64,
    colour: BoxColour,
}

impl ShippingBox {
    fn new() -> Self {
        Self {dimension_x: 32, dimension_y: 20, dimension_z: 15, weight: 34.7, colour: BoxColour::Green}
    }

    fn print_box_characteristics(&self) {
        println!("Dimensions: \n    X: {} \n    Y: {} \n    Z: {}", self.dimension_x, self.dimension_y, self.dimension_z );
        println!("Weight: {}", self.weight);
        match self.colour {
            BoxColour::Brown => println!("Colour: Brown\n"),
            BoxColour::Green => println!("Colour: Green\n"),
        }
    }
}

fn main() {
    let keyboard = ShippingBox {dimension_x: 5, dimension_y: 7, dimension_z: 2, weight: 4.5, colour: BoxColour::Brown};
    keyboard.print_box_characteristics();
    let new_box = ShippingBox::new();
    new_box.print_box_characteristics();
}
