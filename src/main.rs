use std::env::{Args , args };

fn main() {
    let mut _args:Args = args();

    let first = _args.nth(1).unwrap();
    let operator = _args.nth(0).unwrap().chars().next().unwrap();
    let second = _args.nth(0).unwrap(); 

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);

    //println!("{} {} {} {}",first,operator,second,result);

    print!("{:?}",output(first_number, operator, second_number, result));

    // Function to operate or calculate two numbers
    fn operate(operator: char, first_number: f32,second_number: f32)-> f32{

        match operator {
            '+' => first_number+second_number,
            '-' => first_number-second_number,
            '*'|'x' |'X'=> first_number*second_number,
            '/' => first_number/second_number,
            _   => panic!("Invalid operator used")
        }
        // if operator == '+'{
        //     first_number+second_number
        // }
        // else if operator == '-' {
        //     first_number-second_number
        // }else if operator == '/' {
        //     first_number/second_number
        // }else if operator == '*' {
        //     first_number*second_number
        // }else {
        //     0.0
        // }
    }

    fn output(first_number: f32,operator: char,second_number: f32,result: f32) -> String {
        format!("{} {} {} = {}",first_number,operator,second_number,result)
    }

}

