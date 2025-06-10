/*
    Дана строка, состоящая только из цифровых символов. В данной строке
    есть одна цифра, которая не повторяется. Написать функцию,
    которая найдёт эту цифру и вернёт её.

    * Написать похожую функцию, но только на этот раз в данной строке
    могут присутствовать любые символы, а уникальная цифра может отсутствовать.
    Но если присутсвует, то не больше одной. Написать тесты.
*/

fn uniq_digit(s: &str) -> u8 {
    // TODO специально не использую хэш мап, так как еще не проходили.
    let length = s.len();

    for i in 0..length {
        let current = s.chars().nth(i).unwrap();

        let mut is_unique = true;

        for j in 0..length {
            if i == j {
                continue;
            }

            let next = s.chars().nth(j).unwrap();
            if current == next {
                is_unique = false;

                break;
            }
        }

        if is_unique {
            return current.to_digit(10).unwrap() as u8;
        }
    }

    panic!("No unique digits found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(uniq_digit("3"), 3);
        assert_eq!(uniq_digit("010"), 1);
        assert_eq!(uniq_digit("47343077"), 0);
        assert_eq!(uniq_digit("123454321"), 5);
        assert_eq!(uniq_digit("0987654321234567890"), 1);
        assert_eq!(uniq_digit("4444444444424444444444444"), 2);
    }
}

fn main() {
    let result = uniq_digit("3");

    println!("{}", result);
}
