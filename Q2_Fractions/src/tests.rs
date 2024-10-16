use super::*;

#[test]
fn test_neg() {
    let result: Frac = Frac::new(1, 3);
    let expected: Frac = Frac::new(-1, 3);
    assert_eq!(expected, -result);
}

#[test]
fn test_add() {
    let a: Frac = Frac::new(4, 3); // a= 4/3
    let b: Frac = Frac::new(3, 7); // b = 3/7

    let result: Frac = a + b;
    let expected: Frac = Frac::new(37, 21);
    assert_eq!(expected, result);
}

#[test]
fn test_sub() {
    let a: Frac = Frac::new(6, 9); // a = 6/9 = 2/3
    let b: Frac = Frac::new(4, 2); // b = 4/2 = 2/1

    let result: Frac = a - b;
    let expected: Frac = Frac::new(-4, 3);
    assert_eq!(expected, result);
}

#[test]
fn test_mul() {
    let a: Frac = Frac::new(4, 3); // a = 4/3
    let b: Frac = Frac::new(5, 12); // b = 5/12

    let result = a * b;
    let expected = Frac::new(5, 9);
    assert_eq!(expected, result);
}

#[test]
fn test_div() {
    let a: Frac = 1 / Frac::from(3); // a = 1/3
    let b: Frac = Frac::new(1, 6); // b = 1/6

    let result = a / b;
    let expected = Frac::new(2, 1);
    assert_eq!(expected, result);
}

#[test]
fn test_pow() {
    let mut result: Frac = Frac::new(2, 3);
    result = result.pow(5);
    let expected: Frac = Frac::new(32, 243);
    assert_eq!(expected, result);
}

#[test]
fn test_precision() {
    let mut result: Frac = Frac::from(0);
    for i in 2..20 {
        result = result + Frac::new(1, i);
    }
    let expected: Frac = Frac::new(197698279, 77597520);
    assert_eq!(expected, result);
}

#[test]
fn test_conversion() {
    let a: Frac = Frac::from(720);
    let b: Frac = Frac::from(256);

    let result: Frac = a / b;
    let expected: Frac = Frac::new(45, 16);
    assert_eq!(expected, result);

    let expected: Frac = Frac::new(90, 32); // 90/32 = 45/16
    assert_eq!(expected, result);
    
    let result: f64 = f64::from(a/b);
    let expected: f64 = 720.0 / 256.0;
    assert_eq!(expected, result);
}
