use std::ops::RangeInclusive;
use std::default::Default;
use rand::prelude::*;

pub struct ArithmeticTask {
    /// The task as String
    pub task_string: String,
    /// Number of decimal points that the result should be atleast rounded to.
    pub result_decimal_points: u8,
    /// Result (rounded to result_decimal_points)
    pub result: f64,
}

#[derive(PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Operator {
    pub fn priority(&self) -> i32 {
        match *self {
            Operator::Addition => 5,
            Operator::Subtraction => 5,
            Operator::Multiplication => 6,
            Operator::Division => 6,
        }
    }

    pub fn symbol(&self) -> char {
        match *self {
            Operator::Addition => '+',
            Operator::Subtraction => '-',
            Operator::Multiplication => '*',
            Operator::Division => '/',
        }
    }
}

pub struct Config {
    /// has to be var has to be in range [MIN_VARIABLE_NUM, MAX_VARIABLE_NUM]
    pub variable_num: u8,
    pub variable_range: RangeInclusive<i32>,
    /// has to be >= 1. Which of the provided operators are 
    /// choosen for a task is random.
    pub operators: Vec<Operator>,
    pub variable_decimal_points: u8,
    pub result_decimal_points: u8,
    pub num_tasks: u8,
}

impl Config {
    const MAX_VARIABLE_NUM: u8 = 30;
    const MIN_VARIABLE_NUM: u8 = 2;

    pub fn is_valid(&self) -> bool {
        if !(self.variable_num >=  Self::MIN_VARIABLE_NUM
            && self.variable_num <= Self::MAX_VARIABLE_NUM) {
            false
        } else if self.operators.is_empty() {
            false
        } else {
            true
        }
    }

    pub fn generate_new_tasks(&self) -> Vec<ArithmeticTask> {
        let mut result: Vec<ArithmeticTask> = Vec::new();
        for _ in 0..self.num_tasks {
            result.push(self.generate_new_task());
        }
        result
    }

    pub fn generate_new_task(&self) -> ArithmeticTask {
        // get vars
        let mut rng = rand::thread_rng();
        let mut variables: Vec<f64> = Vec::with_capacity(
            self.variable_num as usize);
        let mut operators: Vec<&Operator> = Vec::with_capacity(
            (self.variable_num - 1) as usize);
        let mut task_string = String::new();

        let mut var: f64;
        let mut i = 0;
        let range_start = *self.variable_range.start() as f64;
        let range_end = *self.variable_range.end() as f64;
        while i < self.variable_num {
            var = rng.gen_range(range_start..=range_end);
            var = round(var, self.variable_decimal_points);
            if var == 0f64 {continue;} // skip 0
            variables.push(var);
            i += 1;
        }
        // get ops and task string
        let mut i: usize;
        let mut operator;
        for n in 0..self.variable_num - 1 {
            task_string.push_str(&format!("{}", variables[n as usize]));
            i = rng.gen_range(00..self.operators.len());
            operator = &self.operators[i];
            operators.push(operator);
            task_string.push(' ');
            task_string.push(operator.symbol());
            task_string.push(' ');

        }
        task_string.push_str(&format!("{}", variables[variables.len()-1]));

        // get result
        for _ in 0..operators.len() {
            // find operator mit highest precedence/priority
            let op_idx = max_op(&operators);
            
            // apply operator and write result to operator index
            match *operators[op_idx] {
                Operator::Addition => {
                    variables[op_idx] += variables[op_idx + 1];
                }
                Operator::Subtraction => {
                    variables[op_idx] -= variables[op_idx + 1];
                }
                Operator::Multiplication => {
                    variables[op_idx] *= variables[op_idx + 1];
                }
                Operator::Division => {
                    variables[op_idx] /= variables[op_idx + 1];
                }
            }
              
            // shift all variables operator index
            // one to the left and remove used operator.
            variables.remove(op_idx + 1);
            operators.remove(op_idx);
        }
        ArithmeticTask {
            task_string,
            result_decimal_points: self.result_decimal_points,
            result: round(variables[0], self.result_decimal_points),
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            variable_num: 3,
            variable_range: (-100..=100),
            operators: vec![
                Operator::Addition,
                Operator::Subtraction,
                Operator::Multiplication,
                Operator::Division,
            ],
            variable_decimal_points: 0,
            result_decimal_points: 1,
            num_tasks: 10,
        }
    }
}

fn round(x: f64, points: u8) -> f64 {
    let mut x = x * (10.0f64.powi(points as i32));
    if x < 0f64 {
        x -= 0.5;
    } else {
        x += 0.5;
    }
    let x_int = x as i128;
    (x_int as f64) / 10.0f64.powi(points as i32)
}

// index of operator with highest prio
fn max_op(v: &[&Operator]) -> usize {
    let mut max_idx = 0;
    for idx in 0..v.len() {
        if v[idx].priority() > v[max_idx].priority() {
            max_idx = idx;
        }
    }
    max_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_1() {
        assert_eq!(round(3.454, 2), 3.45);
    }

    #[test]
    fn round_2() {
        assert_eq!(round(-3.454, 2), -3.45);
    }

    #[test]
    fn round_3() {
        assert_eq!(round(4.999, 2), 5f64);
    }

    #[test]
    fn round_4() {
        assert_eq!(round(-4.999, 2), -5f64);
    }

    #[test]
    fn overflow_max() {
        let max_var = i32::MAX as f64;
        let r = max_var.powf(Config::MAX_VARIABLE_NUM as f64);
        assert!(r <= f64::MAX);

    }

    #[test]
    fn overflow_min() {
        let min_var = i32::MIN as f64;
        let r = min_var.powf(Config::MAX_VARIABLE_NUM as f64);
        assert!(r >= f64::MIN && r <= f64::MAX);
    }

}
