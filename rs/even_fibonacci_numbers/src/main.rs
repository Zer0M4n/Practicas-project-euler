
fn main() {
    let mut x: u64 = 1;
    let mut  y: u64 = 1;
    let mut fibonacci: u64 = 0;//fibonachi result
    let mut summation: u64 = 0;//Even numbers sumation

    while fibonacci < 4000000  {
         //fibonacci secuence
         fibonacci = x + y;
         x = fibonacci;
         y = fibonacci-y;
 
         //Even fibonachi numbers summation
         if fibonacci % 2 == 0 {
             summation = summation + fibonacci;
         }
    }

    println!("Even fibonacci numbers summation {}", summation );    

}
