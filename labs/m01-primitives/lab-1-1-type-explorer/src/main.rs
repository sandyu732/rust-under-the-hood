use lab_1_1_type_explorer::{TYPES, predicted_size};
use std::mem::size_of;

fn actual_size(t: &str) -> usize {
    match t {
        "i8" => size_of::<i8>(),
        "i16" => size_of::<i16>(),
        "i32" => size_of::<i32>(),
        "i64" => size_of::<i64>(),
        "i128" => size_of::<i128>(),
        "isize" => size_of::<isize>(),
        "u8" => size_of::<u8>(),
        "usize" => size_of::<usize>(),
        "f32" => size_of::<f32>(),
        "f64" => size_of::<f64>(),
        "bool" => size_of::<bool>(),
        "char" => size_of::<char>(),
        "()" => size_of::<()>(),
        "&str" => size_of::<&str>(),
        "String" => size_of::<String>(),
        "Vec<u8>" => size_of::<Vec<u8>>(),
        "[u8; 16]" => size_of::<[u8; 16]>(),
        "(u8, u32)" => size_of::<(u8, u32)>(),
        "&[u8]" => size_of::<&[u8]>(),
        "Box<i32>" => size_of::<Box<i32>>(),
        "Option<u8>" => size_of::<Option<u8>>(),
        "Option<u32>" => size_of::<Option<u32>>(),
        "Option<bool>" => size_of::<Option<bool>>(),
        "Option<char>" => size_of::<Option<char>>(),
        "Option<&u8>" => size_of::<Option<&u8>>(),
        "Option<Box<i32>>" => size_of::<Option<Box<i32>>>(),
        other => panic!("unknown type {other}"),
    }
}

fn main() {
    println!("{:<18} {:>9} {:>7}", "type", "predicted", "actual");
    for t in TYPES {
        let actual = actual_size(t);
        let predicted = std::panic::catch_unwind(|| predicted_size(t)).ok();
        let mark = if predicted == Some(actual) {
            "✓"
        } else {
            "✗"
        };
        let p = predicted.map_or("todo".to_string(), |p| p.to_string());
        println!("{t:<18} {p:>9} {actual:>7} {mark}");
    }
}
