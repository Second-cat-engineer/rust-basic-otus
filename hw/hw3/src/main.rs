// Вам нужно реализовать программу обработки команд для дисплея.
// На вход пользователь подает:
// * 2 числа: размер дисплея
// * 1 число: цвет дисплея по-умолчанию (1 - красный, 2 - зеленый, 3 - синий)
// * Последовательность команд: набор чисел.
//
// Дисплей поддерживает следующие команды:
// * 1 x y - переместить курсор в позицию x y
// * 2 colour - перекрасить пиксель в цвет colour
//
// Пример входных данных:
// 4 4
// 1
// 1 2 2 2 3
// В результате пиксель по позиции (2,2) будет перекрашен в синий цвет

// Обновлять состояние дисплея нужно через метод matrix.set_colour(pos_x, pos_y, colour)

// Важно! Обязательна проверка на ошибки. Если пользователь просит переместиться на пиксель за пределами дисплея или ввел неправильный цвет, то вам нужно кинуть панику!

use std::io;

#[derive(Debug)]
struct Display {
    // можете добавить сюда любые дополнительные поля
    matrix: Matrix,
}

fn create_display(max_width: u32, max_height: u32, default_colour: u8) -> Display {
    // ваш код сюда
    Display {
        matrix: Matrix::new(max_width, max_height, default_colour),
    }
}

fn process_commands(display: &mut Display, input: Vec<u64>) {
    if input.len() != 5 {
        panic!("Неверное количество аргументов");
    }

    let height = display.matrix.0.len();
    let width = display.matrix.0[0].len();
    // println!("height: {:?}, width: {}", height, width);

    for (idx, &item) in input.iter().enumerate().take(5) {
        if idx == 1 && item as usize > height {
            panic!("Некорректное значение высоты!");
        }
        if idx == 2 && item as usize > width {
            panic!("Некорректное значение ширины!");
        }

        if idx == 4 && !(1..=3).contains(&item) {
            panic!("Некорректное значение цвета!")
        }
    }

    display
        .matrix
        .set_colour(input[2], input[1], input[4] as u8);
}

// код ниже трогать не нужно, можете просто посмотреть его

// тесты
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_case() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 3]);
        let mut expected = Matrix::new(4, 4, 1);
        expected.set_colour(2, 2, 3);
        assert_eq!(display.matrix, expected);
    }

    #[test]
    #[should_panic]
    fn test_error() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 5, 5, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_error_invalid_colour() {
        let mut display = create_display(4, 4, 1);
        process_commands(&mut display, vec![1, 2, 2, 2, 5]);
    }
}

fn main() {
    println!("Введите размеры дисплея (ширина высота):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let (width, height) = parse_dimensions(&input);

    println!("Введите стандартный цвет дисплея (1 - красный, 2 - зеленый, 3 - синий):");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let default_colour = match input.trim() {
        "1" => 1, // Красный
        "2" => 2, // Зеленый
        "3" => 3, // Синий
        _ => panic!("Неверный ввод цвета. Ожидалось 1, 2 или 3."),
    };

    // Создаём дисплей и заполняем его стандартным цветом
    let mut display = create_display(width, height, default_colour);

    // Ввод действий
    println!("Введите строку с действиями:");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let commands = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    // Отображение дисплея
    process_commands(&mut display, commands);

    display.matrix.display();
}

fn parse_dimensions(input: &str) -> (u32, u32) {
    let parts: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Неверный ввод размера"))
        .collect();
    if parts.len() != 2 {
        panic!("Ожидалось два числа для размеров дисплея.");
    }
    (parts[0], parts[1])
}

#[derive(PartialEq, Debug)]
pub struct Matrix(Vec<Vec<u8>>);

fn color_to_char(color: u8) -> char {
    match color {
        1 => '\u{1F534}', // Красный кружок
        2 => '\u{1F7E2}', // Зелёный кружок
        3 => '\u{1F535}', // Синий кружок
        _ => ' ',
    }
}

impl Matrix {
    pub fn new(width: u32, height: u32, default_color: u8) -> Self {
        Self(vec![vec![default_color; width as usize]; height as usize])
    }

    pub fn display(&self) {
        for row in &self.0 {
            for &cell in row {
                print!("{}", color_to_char(cell));
            }
            println!();
        }
    }

    pub fn set_colour(&mut self, x: u64, y: u64, colour: u8) {
        self.0[x as usize][y as usize] = colour;
    }
}
