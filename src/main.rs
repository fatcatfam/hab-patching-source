use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Knock, knock!");

    let mut first_answer = String::new();

    io::stdin().read_line(&mut first_answer)
        .expect("I don't understand a thing you're saying!");


    let mut chars = first_answer.chars();

    match chars.next() {
        Some(a) => match a.cmp(&'W') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'h') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'o') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'s') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'e') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&' ') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'t') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'h') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'e') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'r') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'e') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    match chars.next() {
        Some(a) => match a.cmp(&'?') {
            Ordering::Equal => println!("."),
            _ => println!("!")
        },
        _ => println!("_")
    }

    println!("Banana.");

    println!("Banana Who?");

    println!(".");
    println!(".");
    println!(".");
    println!(".");
    println!(".");
    println!(".");
    println!(".");
    println!(".");

    println!("Banana Fannah Fo-Fannah, I wrote this in my pyjamas!");
}
