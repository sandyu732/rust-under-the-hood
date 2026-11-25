use lab_1_1_type_explorer::predicted_size;
use std::mem::size_of;

macro_rules! check {
    ($($name:literal => $t:ty),* $(,)?) => {
        let mut wrong = Vec::new();
        $(
            let actual = size_of::<$t>();
            let predicted = predicted_size($name);
            if predicted != actual {
                wrong.push(format!("{}: predicted {}, actual {}", $name, predicted, actual));
            }
        )*
        assert!(wrong.is_empty(), "wrong predictions:\n  {}", wrong.join("\n  "));
    };
}

#[test]
fn primitive_sizes() {
    check!("i8" => i8, "i16" => i16, "i32" => i32, "i64" => i64, "i128" => i128,
           "isize" => isize, "u8" => u8, "usize" => usize, "f32" => f32, "f64" => f64,
           "bool" => bool, "char" => char, "()" => ());
}

#[test]
fn compound_sizes() {
    check!("&str" => &str, "String" => String, "Vec<u8>" => Vec<u8>, "[u8; 16]" => [u8; 16],
           "(u8, u32)" => (u8, u32), "&[u8]" => &[u8], "Box<i32>" => Box<i32>);
}

#[test]
fn option_sizes_and_niches() {
    check!("Option<u8>" => Option<u8>, "Option<u32>" => Option<u32>,
           "Option<bool>" => Option<bool>, "Option<char>" => Option<char>,
           "Option<&u8>" => Option<&u8>, "Option<Box<i32>>" => Option<Box<i32>>);
}
