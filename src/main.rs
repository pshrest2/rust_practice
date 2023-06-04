
fn main() {
    let x = {
        let y = 1;
        y + 20
    };

    another_function(x);
}

fn another_function(x: i8){
    println!("The value of x is {x}");
}
