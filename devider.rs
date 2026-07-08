fn main() {
    let mut numb:u64 = 600851475143; // 600_851_475_143;
    let mut divader:u64 = 2;
    
    loop{
        if(numb <= divader){
            break;
        }
        else if(numb % divader == 0){
            numb = numb / divader;
        }
        else{
            divader += 1;
        }
    }
    
    println!("{}", numb);
}
