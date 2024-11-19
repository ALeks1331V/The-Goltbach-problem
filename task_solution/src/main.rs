use std::fs;

fn is_prime(num: usize) -> bool {
    for i in 2..=((num as f64).sqrt() as usize) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn find_sum_of_two_primes(n: usize) -> Option<(usize, usize)> {
    for a in 2..=n {
        if is_prime(a) {
            let b = n - a;
            if b > 0 && is_prime(b) {
                return Some((a, b));
            }
        }
    }
    None
}

fn read_data_from_file() -> usize {
    let string_content = fs::read_to_string("file.txt");
    match string_content{
        Err(_e) => panic!("Reading error"),
        Ok(f) => return f.parse::<usize>().unwrap(),
    }
}

fn write_data_to_file(path: &str, data1: &str, data2: &str){
    let together = format!("{data1}   {data2}");
    fs::write(path, together);
}

fn main() {
    let n = read_data_from_file();
    match find_sum_of_two_primes(n) {
        Some((a, b)) => {
            write_data_to_file("answer.txt", a.to_string().as_str(), b.to_string().as_str());
        },
        None => println!("Нет представления числа {} в виде суммы двух простых чисел", n),
    }
}