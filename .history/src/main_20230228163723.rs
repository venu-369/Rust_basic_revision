#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::collections::HashMap;
use std::ops::Add;

mod restaurant;
use crate::restaurant::order_food;

use std::thread;
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main0() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {} {}", name.trim_end(), greeting);
}

fn main1() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasnt assigned a number");
    age = age + 1;
    println!("I'm {} ND I wanr ${}", age, ONE_MIL);
}

fn main2() {
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
}

fn main3() {
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
}

fn main4() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
}

fn main5() {
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("You are a child");
    } else if (age >= 19) && (age <= 65) {
        println!("You are an adult");
    } else if (age >= 66) && (age <= 120) {
        println!("You are a senior");
    } else {
        println!("You are a ghost");
    }
}

fn main6() {
    let mut my_age: i32 = 47;
    let can_vote: bool = if my_age >= 18 { true } else { false };
    println!("can vote: {}", can_vote);
}

fn main7() {
    let age2 = 125;
    match age2 {
        1..=18 => println!("You are a child"),
        19..=65 => println!("You are an adult"),
        66..=120 => println!("You are a senior"),
        _ => println!("You are a ghost"),
    };
}

fn main8() {
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("You are not old enough to vote"),
        Ordering::Greater => println!("You can vote"),
        Ordering::Equal => println!("You can vote"),
    };
}

fn main9() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr_3 = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    while loop_idx < arr_2.len() {
        println!("val: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_3.iter() {
        println!("val: {}", val);
    }
}

fn main10() {
    let my_tuple: (u8, String, f64) = (47, "Venu".to_string(), 60_000.00);

    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
}

fn main11() {
    let mut st1: String = String::new();
    st1.push('A');
    st1.push_str("word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
}

fn main12() {
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from("words");
    let st8 = st6 + " " + &st7;
    for char in st8.bytes() {
        println!("{}", char);
    }
}

//casting conversions
fn main13() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}

//enum types
fn main14() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekday(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => false,
                _ => true,
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("Monday"),
        Days::Tuesday => println!("Tuesday"),
        Days::Wednesday => println!("Wednesday"),
        Days::Thursday => println!("Thursday"),
        Days::Friday => println!("Friday"),
        Days::Saturday => println!("Saturday"),
        Days::Sunday => println!("Sunday"),
    }
    println!("Is today a weekday? {}", today.is_weekday());
}

//Vector
fn main15() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4, 5];
    vec2.push(6);
    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(2) {
        Some(third) => println!("3rd : {}", third),
        None => println!("There is no third element"),
    }
    for i in &mut vec2 {
        *i += 50;
    }
    for i in &vec2 {
        println!("val : {}", i);
    }
    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    return x + y;
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += val;
    }
    sum
}

fn main16() {
    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of lst = {}", sum_list(&num_list));
}

//Generics

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
fn main17() {
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.4 = {}", get_sum_gen(5.2, 4.6));
}

//OWNERSHIP
//stack: store values in a last in first out format
// Data on the stack must have a defined fixed size
//Heap: When putting data on the heap you request a certain amount of space. the oS finds space available and returns an address for that space called a pointer

//Rules
// 1. Each value has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

fn print_str(x: String) {
    println!("A String  {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A String  {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" Venu");
    println!("Name : {}", name);
}

fn main18() {
    let mut str1 = String::from("World");
    change_string(&mut str1)
}

//HASHMAPS:

fn main19() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Venu");
    heroes.insert("Batman", "Wayne");
    heroes.insert("Spiderman", "Peter");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    if heroes.contains_key(&"Superman") {
        let the_superman = heroes.get(&"Superman");
        match the_superman {
            Some(x) => println!("Superman is a hero"),
            None => println!("Superman is not a hero"),
        }
    }
}

//STRUCTS

fn main20() {
    struct Customer {
        name: String,
        address: String,
        balance: f64,
    }
    let mut bob = Customer {
        name: String::from("Bob"),
        address: String::from("123 Main St"),
        balance: 100.00,
    };
    bob.address = String::from("505 main St");
}

fn main21() {
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

    println!("Area of Rectangle : {}", rec.area());
    println!("Area of Circle : {}", circ.area());
}

//MODULES

//creates: Modules that produce a library or executable
//modules: organize and handle privacy
//Packages: Build, test and share creates
//path: A way of naming an item such as a struct, function

//from the module restaurant
fn main22() {
    order_food();
}

//Error Handling
// Result has 2 variants: Ok and Err
//enum result<T, E> {
//    Ok(T),
//    Err(E),
//}
// where T represents the data type of the value and E represents the type of the error
fn main23() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating the file: {:?}", error)
        }
    };
    write!(output, "Hello, world!\n").expect("Failed to write to the file");

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
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

//ITERATORs

fn main24() {
    let mut arr_it = [1, 2, 3, 4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
}

//CLOSURES

// let var_name = |parameters| -> return_type {BODY}

fn main25() {
    let can_vote = |age: i32| age >= 18;
    println!("Can Vote : {}", can_vote(20));
}

fn main26() {
    let mut samp1 = 5;
    let print_var = || println!("Sampl1 = {}", samp1);
    print_var();
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    print!("samp1 = {}", samp1);
}

fn main27() {
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }

    let sum = |a, b| a + b;
    let prod = |a, b| a * b;
    print!("Sum : {}", use_func(10, 20, sum));
    print!("Prod : {}", use_func(10, 20, prod));
}

//SMART POINTER
//basically an address to the location in the memory
//Box<T> : A pointer to a heap allocated value
//Rc<T> : A reference counted pointer to a heap allocated value
//Ref<T> : A reference counted pointer to a heap allocated value
//RefMut<T> : A reference counted pointer to a heap allocated value

fn main28() {
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
}

//Concurrency
//common problems with parallel programming involve:
//1. Thread are accessing data in the wrong order
// 2. Threads are blocked from executing beacuse of confusion over requiremetns to proceed with execution

fn main29() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread1.join().unwrap();
}

//Real world example

fn main() {
    pub struct Bank {
        balance: f32,
    }
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Insufficient funds: {}", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Withdrawal of {} successful", amt, bank_ref.balance);
        }
    }
    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }
    let bank: &Arc<Mutex<Bank>> = &Arc::new(Mutex::new(Bank { balance: 100.00 }));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| customer(bank_ref))
    });
    for handle in handles {
        handle.join().unwrap();
    }
}
