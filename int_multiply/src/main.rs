use rug::{Assign, Complete, Integer};

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

pub fn int_multiply(x: Integer, y: Integer) -> Integer {
    let is_negative = (x < 0) ^ (y < 0);
    let y_abs_string = y.abs().to_string();
    let x_abs_string = x.abs().to_string();
    let y_digits = y_abs_string
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let x_digits = x_abs_string
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut total = Integer::new();

    for (y_index, y_digit) in (&y_digits).into_iter().rev().enumerate() {
        let mut total_x = Integer::new();
        let mut carry = Integer::new();

        for (x_index, x_digit) in (&x_digits).into_iter().rev().enumerate() {
            let mut tmp = Integer::new();
            tmp.assign((x_digit * y_digit) + &carry);

            let mut digit = Integer::new();
            digit.assign(&tmp % Integer::from(10));

            carry.assign(&tmp / Integer::from(10));

            let mut total_x_tmp = Integer::new();
            total_x_tmp.assign(digit * Integer::u_pow_u(10, x_index as u32).complete());

            let mut new_total_x = Integer::new();
            new_total_x.assign(&total_x + &total_x_tmp);

            total_x.assign(new_total_x);

            if x_index == x_digits.len() - 1 {
                let mut final_carry = Integer::new();
                final_carry.assign(&carry * Integer::u_pow_u(10, x_digits.len() as u32).complete());

                let mut new_total_x_with_carry = Integer::new();
                new_total_x_with_carry.assign(&total_x + &final_carry);

                total_x.assign(new_total_x_with_carry);
            }
        }

        let mut total_tmp = Integer::new();
        total_tmp.assign(&total_x * Integer::u_pow_u(10, y_index as u32).complete());

        let mut new_total = Integer::new();
        new_total.assign(&total + &total_tmp);

        total.assign(new_total);
    }

    if is_negative {
        return total * Integer::from(-1);
    } else {
        return total;
    }
}
