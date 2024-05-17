
fn divide_bit(num: u32) -> Vec<u32> {
    let mut result = Vec::<u32>::new();

    for i in 0..32 {
        if ((num >> i) & 0b1) == 0b1 {
            //println!("{:032b}", 0b1 << i);
            result.push(0b1 << i);
        }
    }
    return result;
}

fn main() {
    let a = 122;
    let divided_nums = divide_bit(a);

    for num in divided_nums {
        println!("0b{:032b}", num);
    }

}
