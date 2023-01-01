#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;

mod restaurant;
use crate::restaurant::order_food;

fn variables() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("Hello, {}! {}", name.trim_end(), greeting);
}

fn constants() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn max_values() {
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
}

fn floating() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
}

fn random_number() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}

fn if_else() {
    let age = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote: {}", can_vote);
}

fn matches_fn() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Equal => println!("Can vote"),
        Ordering::Greater => println!("You gained the right to vote"),
    };
}

fn loops() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st : {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    while_loop();
    for_loop();
}

fn while_loop() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("Arr: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
}

fn for_loop() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    for val in arr_2.iter() {
        println!("Value: {}", val);
    }
}

fn tuple() {
    let my_tuple: (u8, String, f64) = (47, "Derak".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
}

fn strings() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    let st4 = "Random string";
    let mut st5 = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    println!("{}", st7);

    for char in st8.bytes() {
        println!("{}", char);
    }
}

fn casting() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32 = (int_u8 as u32) + (int2_u8 as u32);
}

fn enums() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day "),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }
    println!("Is today the weekend {}", today.is_weekend());
}

fn vector() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    println!("1st : {}", vec2[0]);
    let second = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }
    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec length {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for val in list.iter() {
        sum += val;
    }
    return sum;
}

fn sums() {
    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn generics() {
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5 + 4 = {}", get_sum_gen(5.2, 4.6));
}

fn hashmaps() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    if (heroes.contains_key("Batman")) {
        let the_batman = heroes.get("Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }

    println!("Length: {}", heroes.len());
}

fn structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };
    bob.address = String::from("505 Main St");

    // struct Rectangle<T, U> {
    //     length: T,
    //     height: U
    // }

    // let rec = Rectangle{
    //     length: 4, height: 10.4
    // };

    const PI: f32 = 3.141592;

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area: {}", rec.area());
    println!("Circ Area: {}", circ.area());
}

fn read_write_files() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating file : {:?}", error),
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}

fn iterators() {
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st: {:?}", iter1.next());
}

fn main() {
    // variables()
    // constants()
    // max_values()
    // floating()
    // random_number()
    // matches_fn()
    // loops()
    // tuple()
    // strings()
    // vector()
    // sums()
    // generics()
    // hashmaps()
    // structs()
    // order_food();
    // read_write_files()
    // iterators()
}
