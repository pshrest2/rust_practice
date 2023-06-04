fn main() {
    let mut name = String::from("Pranaya Babu");

    let test = &name;

    let updatable_name = &mut name;
    let len = length(updatable_name);
    println!("Length of {updatable_name} is {len}");

    let updatable_name2 = &mut name;
    let len2 = length(updatable_name2);
    println!("Length of {updatable_name2} is {len2}");

    println!("Value of test is {test}");
}

fn length(variable: &mut String) -> usize {
    variable.push_str(" Shrestha");
    variable.len()
}
