
fn main() {
    let x = {
        let y = 1;
        y + 20
    };

    display_x(x);
    println!("The value of y is{}", calculate_y());
}

fn display_x(x: i8){
    println!("The value of x is {x}");
}

fn calculate_y() -> i32 {
    5
}
