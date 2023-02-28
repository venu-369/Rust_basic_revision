#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

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
fn main() {
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
}
