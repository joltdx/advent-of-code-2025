use super::*;

#[test]
fn test_1() {
    // 50 -> 99 -> 01 without crossing zero
    let count = part2(&[('R', 49), ('L', 98)]);
    assert_eq!(count, 0);
}

#[test]
fn test_2() {
    // 50 -> 99 -> 00 ending up at zero
    let count = part2(&[('R', 49), ('R', 1)]);
    assert_eq!(count, 1);
}

#[test]
fn test_3() {
    // 50 -> 99 -> 00 -> 01 stopping at zero once
    let count = part2(&[('R', 49), ('R', 1), ('R', 1)]);
    assert_eq!(count, 1);
}

#[test]
fn test_4() {
    // 50 -> 01 -> 00 -> 99 stopping at zero once
    let count = part2(&[('R', 49), ('R', 1), ('L', 1)]);
    assert_eq!(count, 1);
}

#[test]
fn test_5() {
    // 50 -> 00 -> and a full rotation ending up at 00 again
    let count = part2(&[('L', 50), ('L', 100)]);
    assert_eq!(count, 2);
}

#[test]
fn test_6() {
    // 50 -> 00 -> and a full rotation ending up at 00 again
    let count = part2(&[('R', 50), ('R', 100)]);
    assert_eq!(count, 2);
}

#[test]
fn test_7() {
    // 50 -> 00 -> and 4 full rotations ending up at 00 again
    let count = part2(&[('L', 50), ('L', 400)]);
    assert_eq!(count, 5);
}

#[test]
fn test_8() {
    // 50 -> 00 -> and 4 full rotations ending up at 00 again
    let count = part2(&[('L', 50), ('R', 400)]);
    assert_eq!(count, 5);
}

#[test]
fn test_9() {
    // 50 and 10 full rotations ending up at 50 again
    let count = part2(&[('R', 1000)]);
    assert_eq!(count, 10);
}
