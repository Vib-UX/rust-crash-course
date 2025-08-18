use scalar::*;

#[test]
fn test_eq() {
    assert!(eq('a', 'a'));
    assert!(eq('b', 'b'));
    assert!(!eq('a', 'b'));
    assert!(!eq('b', 'a'));
}

#[test]
fn test_add() {
    assert_eq!(add(1.0, 2.0, 3.0), 6.0);
    assert_eq!(add(1.0, 2.0, -3.0), 0.0);
}

#[test]
fn test_cast() {
    assert_eq!(cast(1u8, 2i8, 3.0), 6.0);
    assert_eq!(cast(1u8, 2i8, -3.0), 0.0);
}

#[test]
fn test_min_max(){
    assert_eq!(min_max(), Ok((i32::MIN, i32::MAX)));
    let (min, max) = min_max().unwrap();
    assert!(min < 0);
    assert!(max > 0);
    assert_eq!(min, i32::MIN);
    assert_eq!(max, i32::MAX);
}
