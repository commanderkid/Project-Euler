fn main() {
    
    const MAX_NUMB: i64 = 100;
    
    let mut summ_of_squares: i64 = 0;
    let mut sum_of_add: i64 = 0;
    let mut answer: i64 = 0;
    
    for i in 1..=MAX_NUMB {
        summ_of_squares += (i * i);
        sum_of_add += i;
    }
    
    answer = sum_of_add * sum_of_add - summ_of_squares;
    
    println!("{}", answer);
