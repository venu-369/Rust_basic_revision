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

fn main() {
    let mut my_age: i32 = 47;
    let can_vote: bool = if my_age >= 18 { true } else { false };
    println!("can vote: {}", can_vote);
}
