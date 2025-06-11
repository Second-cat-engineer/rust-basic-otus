/*
    Дан массив, который содержит n неповторяющихся чисел в диапазоне
    от 0 до n включительно.

    Написать функцию, которая вернёт единственное число, отсутствующее
    в данном массиве.

    Гарантируется, что числа в массиве не повторяются и все принадлежат
    заданному диапазону.
*/
fn missing_num(nums: &[i32]) -> i32 {
    let length = nums.len();

    for num in 0..length {
        if nums.contains(&(num as i32)) == false {
            return num as i32;
        }
    }

    panic!("nums are empty");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(missing_num(&[1, 2]), 0);
        assert_eq!(missing_num(&[1, 0, 4, 2]), 3);
        assert_eq!(missing_num(&[0, 4, 2, 5, 3, 6]), 1);
    }
}

fn main() {
    let res = missing_num(&[0, 1, 4, 2, 5, 3, 6]);
    println!("res: {}", res);
}
