use lab_5_5_extension_trait::*;

#[test]
fn shout_on_str_and_string() {
    assert_eq!("hi".shout(), "HI!");
    let owned = String::from("rust");
    assert_eq!(owned.shout(), "RUST!"); // String derefs to str
}

#[test]
#[allow(clippy::useless_vec)] // showing that Vec<f64> derefs to [f64]
fn mean_on_slices_and_vecs() {
    assert_eq!([1.0, 2.0, 3.0][..].mean(), Some(2.0));
    assert_eq!(vec![4.0, 6.0].mean(), Some(5.0));
    let empty: Vec<f64> = vec![];
    assert_eq!(empty.mean(), None);
}

#[test]
fn blanket_describe() {
    assert_eq!(42.describe(), "<<42>>");
    assert_eq!(Some("x").describe(), "<<Some(\"x\")>>");
    assert_eq!(vec![1, 2].describe(), "<<[1, 2]>>");
}
