fn main() {
    
    const MAXIMUM: u32 = 4_000_000;
    
    let mut first_fibo: u32 = 0;
    let mut second_fibo: u32 = 1;
    let mut result_fibo: u32 = 0;
    let mut summ: u32 = 0;
    
    loop
    {
        result_fibo =  first_fibo + second_fibo;
        
        if(result_fibo > 4_000_000)
        {
            if(second_fibo % 2 == 0)
            {
                summ += second_fibo;
            }
            break;
        }
        
        if(first_fibo % 2 == 0)
        {
            summ += first_fibo;
        }
        
        first_fibo = second_fibo;
        second_fibo = result_fibo;
    }

    println!("{}", summ);
}
