// Принимает мутабельную ссылку на кортеж и bool значение.
// Если false, возвращает мутабельную ссылку на первый элемент кортежа.
// Если true, возвращает мутабельную ссылку на второй элемент кортежа.
fn tuple_ref(tuple: &mut (String, String), flag: bool) -> &mut String {
    if flag { &mut tuple.1 } else { &mut tuple.0 }
}

// Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
fn mut_slice_ref(slice: &mut [String], num: usize) -> &mut String {
    let len = slice.len();
    if len <= num {
        panic!("panic panic");
    }

    &mut slice[num]
}

// Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
fn slice_ref(slice: &[String], num: usize) -> &String {
    let len = slice.len() - 1;
    if len < num {
        panic!("panic panic");
    }

    let index = len - num;
    &slice[index]
}

// Принимает слайс и число N. Возвращает два слайса с элементами: с нулевого по N-1; с N-го по последний;
fn split_slice(slice: &[u32], num: usize) -> (&[u32], &[u32]) {
    if num == 0 {
        panic!("panic panic");
    }
    let len = slice.len() - 1;
    if len < num {
        panic!("panic panic");
    }

    let split_idx = num - 1;
    slice.split_at(split_idx)
}

// Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
fn split_slice_to_array(slice: &[u32]) -> [&[u32]; 4] {
    let len = slice.len();
    let base = len / 4;
    let rem = len % 4;

    let mut sizes = [base; 4];
    for i in 0..rem {
        sizes[i] += 1;
    }

    let mut result: [&[u32]; 4] = [&[]; 4];
    let mut start = 0;

    for i in 0..4 {
        let end = start + sizes[i];
        result[i] = &slice[start..end];
        start = end;
    }

    result
}

fn main() {
    let mut tuple = (String::from("hello"), String::from("world"));
    let res = tuple_ref(&mut tuple, true);
    println!("{:?}", res);
    // *tuple_ref(&mut tuple, true) = String::from("atatata"); // ("hello", "atatata")

    let mut slice = [String::from("hello"), String::from("world")];
    let res = mut_slice_ref(&mut slice, 0);
    println!("{:?}", res);
    // *mut_slice_ref(&mut slice, 1) = String::from("tututut"); // ["hello", "tututut"]

    let slice = [String::from("hello"), String::from("world")];
    let res = slice_ref(&slice, 0);
    println!("{:?}", res);

    let data: [u32; 5] = [10, 20, 30, 40, 50];
    let (first, second) = split_slice(&data, 3);
    println!("first: {:?}, second: {:?}", first, second);

    let data = [1, 2, 10, 20, 30, 40, 50];
    let res = split_slice_to_array(&data);
    println!("res: {:?}", res);
}
