fn main() {
    let result = fibbonaci(5);
    println!("result: {result}")
}

fn fibbonaci(mut n: u32) -> u32{
    if n == 0 {
        return 0
    }
    let mut first = 0;
    let mut second = 1;
    let mut current_sum = 0;

    loop {
        if n <= 0 {
            return current_sum
        }

        current_sum = first + second;

        first = second;
        second = current_sum;

        n -= 1;
    }
}
