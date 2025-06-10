/*
    Написать функцию, которая будет вычислять произведение цифр числа,
    при этом цифра 0 игнорируется. Затем повторить операцию с результатом
    произведения, пока не получится число, состоящее из одной цифры.
*/

fn digit_product(n: u32) -> u8 {
    if n == 0 {
        return 0;
    }

    let numbers: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut res = digits(&numbers);
    while res >= 10 {
        let numbers: Vec<u32> = res
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        res = digits(&numbers);
    }

    res as u8
}

fn digits(numbers: &Vec<u32>) -> u32 {
    let mut result: u32 = 1;
    for &num in numbers {
        if num == 0 {
            continue;
        }
        result *= num;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(digit_product(0), 0);
        assert_eq!(digit_product(9), 9);
        assert_eq!(digit_product(10), 1);
        assert_eq!(digit_product(987), 2); // 9*8*7=504, 5*4=20, 2
        assert_eq!(digit_product(123456), 4); // 1*2*3*4*5*6=720, 7*2=14, 1*4=4
        assert_eq!(digit_product(123454321), 6); // 1*2*3*4*5*4*3*2*1=2880, 2*8*8=128, 1*2*8=16, 1*6=6
    }
}

fn main() {
    let result = digit_product(123456);

    println!("{}", result);
}
