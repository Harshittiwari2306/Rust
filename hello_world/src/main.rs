fn main() {
    // println!("Hello, world!");


    // data_types in Rust
    // let num:u8 = 6; // it is immutable by default
    // println!("The number is: {}", num);


    // let mut mutable_num:u8 = 10; // mutable variable
    // println!("The mutable number is: {}", mutable_num);
    // mutable_num = 15;
    // println!("The updated mutable number is: {}", mutable_num);  


    // strings in rust
    let sentence: &str = "This is a string slice"; // &str type
    // sentence.push_str(" with more text added."); // This will cause a compile-time error
    println!("The sentence is: {}", sentence);


    let mut string_object: String = String::from("This is a string object"); // String type
    string_object.push_str(" with more text added.");
    println!("The string object is: {}", string_object);


    // Tuple in rust

    let emp_info: (&str,u8) = ("Kane", 22);

    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    // destructuring in rust 

    let (employee_name,employee_age) = emp_info;
    println!("Employee Name = {}, Age: {}", employee_name, employee_age);
    println!("Employee Name: {}, Age: {}", emp_name, emp_age);
}
