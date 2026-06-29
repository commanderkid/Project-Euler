fn main() {
    let mut sum_of_all_square_odd_numbers: u128 = 0;
    
    for i in (1u128..=828_000).step_by(2) {
        sum_of_all_square_odd_numbers += i * i;
    }
    
    println!("{}", sum_of_all_square_odd_numbers);
}
