use std::iter::Product;

fn palindrome(text : String) -> bool {
    
    return text.chars().eq(text.chars().rev());
}
fn palindorme_product()  {
    let mut number : u32 = 100;
    let mut  number2 : u32 = 100;
    let mut  product : u32;
    let mut  Product2 : u32;
    
    let mut number_text: String;

    while number <= 999 {
        
        
        while number2 <= 999 {

            product = number * number2;
            number_text = product.to_string();

            if palindrome(number_text) == true {

                println!("is a palindrome = {}",product);
                
            }



            number2 = number2 + 1;
        }

        number = number + 1;
        number2 = 100;
    }


}

fn main() {
    palindorme_product();
}
