use std::io;

fn main() {

    loop {
        
        println!("\n Press 0 : To exit!");
        println!("\n Press 1 : To convert F to C"); 
        println!("\n Press 2 : To convert C to F");
        
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\n You chose: {}", choice);

        if choice == 0 {
            println!("\n Exiting Bye-Bye!! \n");
            break;
        }
        if choice == 1 {
            println!("\n Enter temp. in Fahrenheit:");

            let mut fahrenheit  = String::new();

            io::stdin().read_line(&mut fahrenheit)
                .expect("Failed to read line");
    
            let fahrenheit: f64 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let celcius = (fahrenheit - 32.0)*0.55; 
            println!("\n {} Fahrenheit in Celcius is: {}",fahrenheit,celcius);

        }

        if choice == 2 {
            println!("\n Enter temp. in celcius:");
            
            let mut celcius_new  = String::new();

            io::stdin().read_line(&mut celcius_new)
                .expect("Failed to read line");
    
            let celcius_new: f64 = match celcius_new.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("\n Temp in celcius_new is: {}",celcius_new);
            
            let fahrenheit_new = (celcius_new/0.55) + 32.0;
            println!("\n {} Celcius in Fahrenheit is: {}",celcius_new,fahrenheit_new);

        }

    }
    
}
