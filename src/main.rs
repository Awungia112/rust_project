mod calculator;
use std::env;

fn main() {
    let calculator = calculator; 

    let args: Vec<String> = env::args().collect();

    let operand1 = args[1].parse::<f64>().unwrap();
    let operand2 = args[3].parse::<f64>().unwrap();
    let operator = &(*args[2]);

    match operator{
        "+" =>{
            println!("{}" ,calculator.add(&operand1,&operand2));
        }

        "-" =>{
            println!("{}" ,calculator.subtract(&operand1,&operand2));
        }

        "/" =>{
            println!("{}" ,calculator.divide(&operand1,&operand2));
        }
       
        "*" =>{
            println!("{}" ,calculator.multiply(&operand1,&operand2));
        }

        "%" =>{
            println!("{}" ,calculator.modulus(&operand1,&operand2));
        }

        "powf" =>{
            println!("{}" ,calculator.power(&operand1,&operand2));
        }

        "!" =>{
            println!("{}" ,calculator.factorial(&operand1,&operand2));
        }



        "_" => {
            println!("enter + / * ^ % ! -")
        }
    }
}























