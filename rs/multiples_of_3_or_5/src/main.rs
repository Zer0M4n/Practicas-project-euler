fn main() {
    let mut  summation : u32 = 0;
    
    for natural_numbers  in 0..=999  {//Numbers bellow 1000
        
        if natural_numbers % 3 == 0 || natural_numbers % 5 == 0 {
            summation = summation + natural_numbers;
        }
        
    }
    println!( "{}" , summation );
}
