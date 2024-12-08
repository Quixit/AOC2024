use std::fs;

fn get_input() -> Vec<Vec<u64>> {

    let contents: Vec<String> = fs::read_to_string("./D07/input.txt")
        .expect("File exists.").replace("\r\n", "\n")
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let result: Vec<Vec<u64>> =  contents.iter()
        .map(|l| l.replace(":", "")
            .split(' ')
            .map(|v| v.parse::<u64>()
            .unwrap())
            .collect()
        )
        .collect();

    return  result;
}


fn main() {
    let input = get_input();      
    let mut sum: u64 = 0; 
    let mut sum_two: u64 = 0;

    for equation in input {
        let one = evaluate_equation(equation[1..equation.len()].to_vec(), false).contains(&equation[0]);
        let two = evaluate_equation(equation[1..equation.len()].to_vec(), true).contains(&equation[0]);

        if one  {
            sum += equation[0];
        }

        if two {
            sum_two += equation[0];
        }
    }

    println!("Calibration Result: {sum}");
    println!("Calibration Result Two: {sum_two}");
}

fn evaluate_equation(equation: Vec<u64>, q2: bool) -> Vec<u64> {
    if equation.len() == 1 {
        return [equation[0]].to_vec();
    }   

    let mut equation_one = [equation[0] + equation[1]].to_vec();   
    let mut rest =  equation.clone()[2..equation.len()].to_vec();

    equation_one.append(&mut rest);

    let mut equation_two = [equation[0] * equation[1]].to_vec();   
    let mut rest =  equation.clone()[2..equation.len()].to_vec();

    equation_two.append(&mut rest);

    let mut result = evaluate_equation(equation_one, q2);
    let mut result_two = evaluate_equation(equation_two, q2);
    result.append(&mut result_two);

    if q2
    {
        let one = equation[0];
        let two = equation[1];
        let format:u64 = format!("{one}{two}").parse().unwrap();
        let mut equation_three = [format].to_vec();   
        let mut rest =  equation.clone()[2..equation.len()].to_vec();
    
        equation_three.append(&mut rest);

        let mut result_three = evaluate_equation(equation_three, q2);
        result.append(&mut result_three);
    }

    return result
}