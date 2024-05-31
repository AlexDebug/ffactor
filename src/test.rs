use super::*;

#[test]
fn test_even() {
    let even: [u128; 5] = [2, 4, 8, 6, 14];
    let odd: [u128; 5] = [3, 5, 7, 9, 19];
    for i in 0..5 {
        assert_eq!(calc::is_even(even[i]), true);
        assert_eq!(calc::is_even(odd[i]), false)
    }
 }
#[test]
fn test_square() {
    let square: [u128; 3] = [4, 9, 16];
    let no_square: [u128; 3] = [6, 8, 15];
    for i in 0..3 {
        assert_eq!(calc::is_square(square[i]), true);
        assert_eq!(calc::is_square(no_square[i]), false)
    }
}