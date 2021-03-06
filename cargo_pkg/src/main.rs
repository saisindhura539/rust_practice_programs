use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    const PRINT_IT: u32 = 15;

    // casting 
    println!("value of  literal is {}",(PRINT_IT as f32) /(2.0));
    // vector creation and insert(push)

    let mut v: Vec<f32> = Vec::new(); // creates empty new vector
    v.push(1.0);
    v.push(2.0);
    //v.push(3);
    //v.push(4);

    // retreiving out of bound value. Safe to use
    let _x = v.get(4);
    println!("{:#?}",_x);  // Prints None. Safe to use get()

    // UTF-8 encoded string(can store 65536 chars) unlike ascii(255)
    let _hello = String::from("नमस्ते");

    // println!("hello {}!",&hello[0..2]); // runtime rust crash
    
    let mut y = HashMap::new(); // creates empty hashmap
    y.insert(String::from("hola!"),String::from("ooo"));
    y.insert(String::from("bond"),String::from("James"));
    //accessing values through keys
    let z = y.get("bond");
    
    // prints hashmap
    println!("{:#?}",y);

    // if-else
    let z: bool = false;
    if z
    {
        println!("IF it is {}",z);
    }
    else {
        println!("Else it is {}",z);
    }

    // slices(reference)
    let _hellow = String::from("hello world");
    let first_slic = &_hellow[0..5];
    println!("{}",_hellow);
    // compare
    assert_eq!(first_slic,"hello");


    // for loop
    for i in v.iter() {
        println!("{}",i);
    }
    for i in _hello.chars() {
        println!("{}",i);
    }

    // match
    let binary: bool = true;
    match binary {
        true => println!("true...yes"),
        false => println!("false"),
    }

    // function calling
    fizzbuzz(5);

}


fn fizzbuzz(n: u32) -> (bool,bool) {
    for i in 1..=n {
    println!("In fizzbuzz");
    }
    (true,false) // return multiple values--tuple
}
