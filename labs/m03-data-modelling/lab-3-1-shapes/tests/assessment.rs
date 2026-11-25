use lab_3_1_shapes::Shape::*;
use lab_3_1_shapes::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn areas() {
    assert!(close(Circle { r: 1.0 }.area(), std::f64::consts::PI));
    assert!(close(Rect { w: 2.0, h: 3.0 }.area(), 6.0));
    assert!(close(
        Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0
        }
        .area(),
        6.0
    ));
}

#[test]
fn perimeters() {
    assert!(close(Circle { r: 0.5 }.perimeter(), std::f64::consts::PI));
    assert!(close(Rect { w: 2.0, h: 3.0 }.perimeter(), 10.0));
    assert!(close(
        Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0
        }
        .perimeter(),
        12.0
    ));
}

#[test]
fn validity() {
    assert!(Circle { r: 1.0 }.is_valid());
    assert!(!Circle { r: 0.0 }.is_valid());
    assert!(!Rect { w: -1.0, h: 2.0 }.is_valid());
    assert!(
        !Triangle {
            a: 1.0,
            b: 2.0,
            c: 3.0
        }
        .is_valid()
    );
    assert!(
        Triangle {
            a: 2.0,
            b: 2.0,
            c: 3.0
        }
        .is_valid()
    );
}

#[test]
fn scaling_mutates_in_place() {
    let mut s = Rect { w: 1.0, h: 2.0 };
    s.scale(3.0);
    assert_eq!(s, Rect { w: 3.0, h: 6.0 });
    let mut t = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    t.scale(2.0);
    assert!(close(t.area(), 24.0));
}

#[test]
fn kinds_and_largest() {
    let shapes = vec![
        Rect { w: 1.0, h: 1.0 },
        Circle { r: 2.0 },
        Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0,
        },
    ];
    assert_eq!(largest(&shapes).map(Shape::kind), Some("circle"));
    assert_eq!(largest(&[]), None);
}
