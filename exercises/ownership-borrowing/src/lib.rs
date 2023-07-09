// Exercise 1
// Make it compile
fn exercise1() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y :  &str= &x;
    let z = x;
}

// Exercise 2
// Make it compile
// Don't modify code in exercise2 function!
fn exercise2() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s // Return the owned string
}


// Exercise 3
// Make it compile
// Dont care about logic
fn exercise3() {
    let values: Vec<f64> = vec![
        2817.42, 2162.17, 3756.57, 2817.42, -2817.42, 946.9, 2817.42, 964.42, 795.43, 3756.57,
        139.34, 903.58, -3756.57, 939.14, 828.04, 1120.04, 604.03, 3354.74, 2748.06, 1470.8,
        4695.71, 71.11, 2391.48, 331.29, 1214.69, 863.52, 7810.01,
    ];

    let values_number = values.len();

    let additions: Vec<usize> = vec![0];

    println!("{:?}", values_number);

    while !additions.is_empty() {
        let mut addition: f64 = 0.0;

        // Sum values in additions
        for &element_index in &additions {
            let addition_aux = values[element_index];
            addition = addition_aux + addition;
        }

        // Update additions or exit loop
        break;
    }
}

// Exercise 4
// Make it compile
fn exercise4(value: u32) -> &'static str {
    let str_value = value.to_string();
    let static_ref: &'static str = Box::leak(str_value.into_boxed_str());
    static_ref
}


// Exercise 5
// Make it compile
use std::collections::HashMap;

fn exercise5() {
    let mut my_map = HashMap::from([(1, "1.0".to_string()), (2, "2.0".to_string())]);

    let key = 3;

    let res = match my_map.get(&key) {
        Some(child) => child,
        None => {
            let value = "3.0".to_string();
            my_map.entry(key).or_insert(value.clone());
            my_map.get(&key).unwrap()
        }
    };

    println!("{}", res);
}

// Exercise 6
// Make it compile

use std::io;

fn exercise6() {
    let mut prev_key = String::new();

    for line in io::stdin().lines() {
        let s = line.unwrap();

        let data: Vec<&str> = s.split('\t').collect();
        if prev_key.is_empty() {
            prev_key = String::from(data[0]);
        }
    }
}

// Exercise 7
// Make it compile
fn exercise7() {
    let mut v: Vec<String> = Vec::new();
    {
        let chars = [b'x', b'y', b'z'];
        let s: String = String::from_utf8(chars.to_vec()).unwrap();
        v.push(s);
    }
    println!("{:?}", v);
}

// Exercise 8
// Make it compile
use std::io;

fn exercise8() {
    let mut accounting = vec!["Alice", "Ben"];

    loop {
        let mut add_input = String::new();

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim().split_whitespace().collect();

        if add_vec.is_empty() {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person);

        if person == "quit" { // Add a condition to break the loop
            break;
        }
    }
}
