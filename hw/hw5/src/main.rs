// Слайсы.
// Ring Buffer (кольцевой буффер) - структура данных, которая позволяет очень удобно реализовывать очередь на массиве фиксированного размера.
// https://ru.wikipedia.org/wiki/%D0%9A%D0%BE%D0%BB%D1%8C%D1%86%D0%B5%D0%B2%D0%BE%D0%B9_%D0%B1%D1%83%D1%84%D0%B5%D1%80
// Ключевая идея в том, что заполняя буффер до конца мы переходим в начало
// Пример API, вызовов и как меняется состояние буффера:
// [ _ _ _ ] create(3)
// [ a b _ ] write "ab" -> return 2
// [ a b c ] write "cd" -> return 1
// [ _ b c ] read(1) -> return "a"
// [ e b c ] write "e" -> return 1
// [ e _ _ ] read(2) -> return "bc"
// Ваша задача написать такой буффер и добавить тесты

#[derive(Debug)]
struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    data: Vec<u8>,
    is_full: bool,
}

fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        data: vec![0; size],
        is_full: false,
    }
}

// Да, в базовой версии просто откидываем данные, которые не поместились (и возвращаем кол-во байт, которые удалось записать)
fn write(rb: &mut RingBuffer, data: &str) -> usize {
    let mut write_bytes = 0;
    let input_bytes = data.as_bytes();

    for &byte in input_bytes {
        if rb.is_full {
            break;
        }

        rb.data[rb.write_idx] = byte;

        write_bytes += 1;

        rb.write_idx += 1;
        if rb.write_idx == rb.data.len() {
            rb.write_idx = 0;
        }

        if rb.write_idx == rb.read_idx {
            rb.is_full = true;
        }
    }

    write_bytes
}

fn read(rb: &mut RingBuffer, elem_count: usize) -> String {
    let mut result = Vec::new();
    let mut read = 0;

    while rb.read_idx != rb.write_idx || rb.is_full {
        if read >= elem_count {
            break;
        }

        result.push(rb.data[rb.read_idx]);
        rb.data[rb.read_idx] = 0;

        rb.read_idx += 1;
        if rb.read_idx == rb.data.len() {
            rb.read_idx = 0;
        }

        rb.is_full = false;

        read += 1;
    }

    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut rb = create(3);
        assert_eq!(write(&mut rb, "safuan"), 3);
        assert_eq!(read(&mut rb, 2), "sa");
        assert_eq!(write(&mut rb, "ra"), 2);
        assert_eq!(read(&mut rb, 3), "fra");
        assert_eq!(read(&mut rb, 3), "");
    }
}

fn main() {
    let mut rb = create(3);
    println!("{:?}", rb);
    write(&mut rb, "asafff");
    println!("{:?}", rb);

    let readed = read(&mut rb, 2);
    println!("{:?}", readed);
    println!("{:?}", rb);

    let readed = read(&mut rb, 2);
    println!("{:?}", readed);
    println!("{:?}", rb);

    write(&mut rb, "ddd");
    println!("{:?}", rb);

    let readed = read(&mut rb, 1);
    println!("{:?}", readed);
    println!("{:?}", rb);
}
