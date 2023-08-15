#![allow(non_snake_case)]

pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn it_works() {
    let mut x = "hello world";
    strtok(&mut x, ' ');
    println!("{}", x);

    let nums = vec![1, 2];
    let temp = &*nums;
    print_type_of(&temp);

    let _aaa:[i32;5] = [1,2,3,4,5];
    // let _bbb:[String;5] = ['a', 'b', 'c', 'd', 'e'];
    let _ccc = [1;5];
    let _ddd = ['a', 'b', 'c', 'd', 'e'];
}

pub fn main() { it_works(); }