/*
    Написать функцию, которая превращает число в строку по следующим правилам:
    1. Если число кратно 3, то возвращаем строку "Fizz"
    2. Если число кратно 5, то возвращаем строку "Buzz"
    3. Если число кратно и 3, и 5, то возвращаем строку "FizzBuzz"
    4. В остальных случаях возвращаем строку, содержащую данное число

    Написать функцию fizzbuzz_list, которая получает число `n: u32` и возвращает
    список строк, содержащих строковые представления fizzbuzz
    для чисел в диапазоне от 1 до n. Написать тесты.
*/

fn fizzbuzz_list(n: u32) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();

    for num in 1..=n {
        let result = fizzbuzz(num);

        list.push(result);
    }

    list
}

#[cfg(test)]
mod fizzbuzz_list_tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_list_up_to_5() {
        let result = fizzbuzz_list(5);
        let expected = vec!["1", "2", "Fizz", "4", "Buzz"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(result, expected);
    }
}

fn fizzbuzz(num: u32) -> String {
    if num % 3 == 0 && num % 5 == 0 {
        String::from("FizzBuzz")
    } else if num % 3 == 0 {
        String::from("Fizz")
    } else if num % 5 == 0 {
        String::from("Buzz")
    } else {
        num.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(&fizzbuzz(1), "1");
        assert_eq!(&fizzbuzz(3), "Fizz");
        assert_eq!(&fizzbuzz(5), "Buzz");
        assert_eq!(&fizzbuzz(7), "7");
        assert_eq!(&fizzbuzz(9), "Fizz");
        assert_eq!(&fizzbuzz(15), "FizzBuzz");
        assert_eq!(&fizzbuzz(30), "FizzBuzz");
        assert_eq!(&fizzbuzz(49), "49");
    }
}

fn main() {
    let result = fizzbuzz_list(15);

    println!("{:?}", result);
}
