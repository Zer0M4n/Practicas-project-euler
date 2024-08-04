fn smallest_multiple() {
    let mut  x : u32 = 1;
    let mut  i: u32 = 1;

    loop {
        
        while i < 20 {
            
            if x % i == 0 {
                i = i + 1;
            }else {
                break;
            }

        }

        if i == 20 {
            break;
        }

        x = x +1;
        
        i = 1;
    }

    println!("{}",x)
    
}
fn main() {
    smallest_multiple();
    println!("Hello, world!");
}
