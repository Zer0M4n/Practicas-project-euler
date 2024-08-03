fn primefactos(mut number_prime: f64 )  {
    let mut  i: f64 = 3.0;

    while number_prime % 2.0 == 0.0 {
        println!("2");

        number_prime = number_prime / 2.0;
        
    }

    while i <= number_prime.sqrt(){

        while number_prime % i == 0.0 {
            println!("{}",i);


            number_prime = number_prime / i;
        }

        i = i + 2.0;  

    }

    if number_prime > 2.0 {
            
        println!("{}",number_prime);

    }

     
}
fn main() {
    let  number_prime: f64 = 600851475143.0;
    
    primefactos(number_prime);
    
}
