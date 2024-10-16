use super::*;

#[test]
fn test_bonus_add() {
    let mut a: Frac =Frac::new(4, 3); // a= 4/3
    let b: Frac = Frac::new(3, 7); // b = 3/7

    a += 7;
    a += b;
    let expected: Frac = Frac::new(184, 21);
    assert_eq!(expected, a);
}

#[test]
fn test_bonus_sub() {
    let mut a: Frac = Frac::new(6, 9); // a = 6/9 = 2/3
    let b: Frac = Frac::new(4, 2); // b = 4/2 = 2/1

    a -= 7;
    a -= b;
    let expected: Frac = Frac::new(-25, 3);
    assert_eq!(expected, a);
}

#[test]
fn test_bonus_mul() {
    let mut a: Frac = Frac::new(4, 3); // a = 4/3
    let b: Frac = Frac::new(5, 12); // b = 5/12

    a *= 7;
    a *= b;
    let expected: Frac = Frac::new(35, 9);
    assert_eq!(expected, a);

    a *= 1;
    assert_eq!(expected, a);

    a *= 0;
    let expected: Frac = Frac::from(0);
    assert_eq!(expected, a);
}

#[test]
fn test_bonus_div() {
    let mut a: Frac = Frac::new(1, 3); // a = 1/3
    let b: Frac = Frac::new(1, 6); // b = 1/6

    a /= 11;
    a /= b;
    let expected: Frac = Frac::new(2, 11);
    assert_eq!(expected, a);

    a /= 1;
    assert_eq!(expected, a);

    a /= a;
    let expected: Frac = Frac::from(1);
    assert_eq!(expected, a);
}

#[test]
fn test_bonus_comparision() {
    let a: Frac = Frac::new(4, 3);
    let b: Frac = Frac::new(9, 7);

    assert!(a > b);
    assert!(a >= b);
    assert!(b < a);
    assert!(b <= a);
    assert!(a != b);

    assert!(std::cmp::max(a, b) == a);
    assert!(std::cmp::min(a, b) == b);
}
