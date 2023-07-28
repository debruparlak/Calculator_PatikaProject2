#![allow(non_snake_case)]
use std::io::stdin;
use std::str::FromStr;
fn main() {
    loop{
    let mut num1_in =String::new();
    let mut num2_in:String=String::new();
    let mut op_in=String::new();
    
    println!("Select the operation you want to perform
    + for addition
    - for subtraction
    * for multiplication
    / for division
    x to exit");//getting the operation type from user
    stdin().read_line( &mut op_in).unwrap();
    let operation=op_in.trim();
    let operator;
    if operation == "x"{
        break;
    }//exiting the calculator
        
    println!("Enter the first number: ");
    stdin().read_line(&mut num1_in).unwrap();
    let num1:f64=num1_in.trim().parse::<f64>().unwrap();//change string to float
    
    println!("Enter the second number: ");
    stdin().read_line(&mut num2_in).unwrap();
    let num2:f64 =num2_in.trim().parse::<f64>().unwrap();//change string to float
    
    match  operation{
        "+" =>{
            operator=Operation::Add(num1, num2)
        },
        "-" =>{
            operator=Operation::Subtract(num1, num2)
        },
        "*" =>{
            operator=Operation::Multiply(num1, num2)
        },
        "/" =>{
            operator=Operation::Divide(num1, num2)
        },
        _  =>{
            println!("Wrong operator please try again!");
            continue;
        }
    };
    let result=calculate(operator);
    println!("Result{result}");//printing the result
    }
}

enum Operation {
    Add (f64,f64),
    Subtract (f64,f64),
    Multiply (f64,f64),
    Divide (f64,f64),
}
fn calculate(operation : Operation)->f64{
    match operation {
        Operation::Add(num1,num2)=>{
            return num1 + num2
        }
        Operation::Subtract(num1,num2)=>{
            return num1 - num2
        }
        Operation::Multiply(num1,num2)=>{
            return num1 * num2
        }
        Operation::Divide(num1,num2)=>{
            return num1 / num2
        }
    }
}

