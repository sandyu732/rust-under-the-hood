use lab_5_4_top_k::*;

fn staff() -> Vec<Employee> {
    vec![
        Employee::new("Ana", "eng", 150),
        Employee::new("Bo", "sales", 90),
        Employee::new("Cy", "eng", 180),
        Employee::new("Di", "eng", 150),
        Employee::new("Ed", "ops", 120),
        Employee::new("Fa", "sales", 200),
    ]
}

fn names<'a>(v: impl IntoIterator<Item = &'a Employee>) -> Vec<&'a str> {
    v.into_iter().map(|e| e.name.as_str()).collect()
}

#[test]
fn ord_is_dept_then_salary_desc_then_name() {
    let s = sorted(staff());
    assert_eq!(names(&s), ["Cy", "Ana", "Di", "Ed", "Fa", "Bo"]);
    assert!(Employee::new("Z", "a", 1) < Employee::new("A", "b", 1));
}

#[test]
fn top_k() {
    let s = staff();
    assert_eq!(names(top_k_earners(&s, 3)), ["Fa", "Cy", "Ana"]);
    assert_eq!(names(top_k_earners(&s, 4)), ["Fa", "Cy", "Ana", "Di"]);
    assert!(top_k_earners(&s, 0).is_empty());
    assert_eq!(top_k_earners(&s, 100).len(), 6);
}
