/*
    Дана строка, состоящая только из символов '{', '}', '(', ')', '[', ']'.
    Такая строка является корректной, если:
    - каждой открывающей скобке соответствует закрывающая того же типа
    - соблюдается порядок закрытия скобок
    - для каждой закрывающей скобки есть соответствующая открывающая пара

    Написать функцию, которая проверит корректность данной строки.
*/
fn validate_paren(s: &str) -> bool {
    let length = s.len();
    if length % 2 != 0 {
        return false;
    }

    let curly_tuple = ('}', '{');
    let parenthesis_tuple = (')', '(');
    let square_tuple = (']', '[');

    let mut chars: Vec<char> = Vec::new();

    for i in 0..length {
        let char = s.chars().nth(i).unwrap();
        // println!("{}", char);

        if char == curly_tuple.0 {
            let last = chars.pop().unwrap();
            if last != curly_tuple.1 {
                return false;
            }
        } else if char == parenthesis_tuple.0 {
            let last = chars.pop().unwrap();
            if last != parenthesis_tuple.1 {
                return false;
            }
        } else if char == square_tuple.0 {
            let last = chars.pop().unwrap();
            if last != square_tuple.1 {
                return false;
            }
        } else {
            chars.push(char);
        }
    }

    chars.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_paren("()"), true);
        assert_eq!(validate_paren("()[]{}"), true);
        assert_eq!(validate_paren("({[]()})"), true);
        assert_eq!(validate_paren("(}"), false);
        assert_eq!(validate_paren("()]"), false);
        assert_eq!(validate_paren("(){"), false);
    }
}

fn main() {
    let res = validate_paren("({[]()})");
    println!("{:#?}", res);
}
