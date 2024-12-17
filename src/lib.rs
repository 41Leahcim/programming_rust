pub const fn gcd(mut left: u64, mut right: u64) -> u64 {
    assert!(left != 0 && right != 0);
    while right != 0 {
        if right < left {
            (left, right) = (right, left);
        }
        right %= left;
    }
    left
}

#[test]
fn test_gcd() {
    assert_eq!((const { gcd(14, 15) }), 1);
    assert_eq!(
        (const { gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19) }),
        33
    );
}
