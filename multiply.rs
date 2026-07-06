const MAX_NUMBER: u32 = 1000;

fn main() {
    
    let mut summa: u32 = 0;
    
    for i in (0..MAX_NUMBER).step_by(3) {
        
        if(i % 5 == 0)
        {
            continue;
        }
            
        summa += i;
    }
    
    for j in (0..MAX_NUMBER).step_by(5) {
        summa += j;
    }
    
    println!("{}", summa);
}
