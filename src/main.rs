use rug::{Assign, Integer};

mod int_multiply;

use int_multiply::int_multiply;

fn main() {
    assert_eq!(
        int_multiply(Integer::from(12), Integer::from(34)),
        Integer::from(12 * 34)
    );

    let mut int = Integer::new();
    let mut int2 = Integer::new();
    let mut decimal = "9";
    int.assign(Integer::parse(decimal).unwrap());
    int2.assign(Integer::parse(decimal).unwrap());

    assert_eq!(
        int_multiply(int.clone(), int2.clone()),
        int.clone() * int2.clone()
    );

    decimal = "123456789";
    int.assign(Integer::parse(decimal).unwrap());
    int2.assign(Integer::parse(decimal).unwrap());

    assert_eq!(
        int_multiply(int.clone(), int2.clone()),
        int.clone() * int2.clone()
    );

    let long_decimal_1 = "3141592653589793238462643383279502884197169399375105820974944592";
    let long_decimal_2 = "2718281828459045235360287471352662497757247093699959574966967627";
    int.assign(Integer::parse(long_decimal_1).unwrap());
    int2.assign(Integer::parse(long_decimal_2).unwrap());

    assert_eq!(
        int_multiply(int.clone(), int2.clone()),
        int.clone() * int2.clone()
    );

    assert_eq!(int_multiply(Integer::from(3), Integer::from(0)), 3 * 0);

    assert_eq!(
        int_multiply(Integer::from(-323), Integer::from(123)),
        -323 * 123
    );

    assert_eq!(
        int_multiply(Integer::from(-999), Integer::from(-888)),
        999 * 888
    );
}
