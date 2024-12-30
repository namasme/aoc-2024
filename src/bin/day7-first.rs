use std::fs;

fn main() {
    let input = fs::read_to_string("data/day7/input").unwrap();
    let equations = parse_input(&input);
    let calibration_result = compute_calibration_result(&equations);
    println!("{}", calibration_result);
}

fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(|line| Equation::from(line)).collect()
}

fn compute_calibration_result(equations: &[Equation]) -> Value {
    equations
        .iter()
        .filter(|equation| equation.is_satisfiable())
        .map(|equation| equation.test_value)
        .sum()
}

type Value = u64;

struct Equation {
    test_value: Value,
    operands: Vec<Value>,
}

impl Equation {
    fn is_satisfiable(&self) -> bool {
        let mut pending = vec![PartialSolution {
            current_value: 0,
            remaining_operands: &self.operands,
        }];

        while let Some(partial_solution) = pending.pop() {
            if partial_solution.is_complete() {
                if partial_solution.current_value == self.test_value {
                    return true;
                } else {
                    continue;
                }
            }
            let candidates = vec![
                PartialSolution {
                    current_value: partial_solution.current_value
                        + partial_solution.remaining_operands[0],
                    remaining_operands: &partial_solution.remaining_operands[1..],
                },
                PartialSolution {
                    current_value: partial_solution.current_value
                        * partial_solution.remaining_operands[0],
                    remaining_operands: &partial_solution.remaining_operands[1..],
                },
            ];

            pending.extend(
                candidates
                    .into_iter()
                    .filter(|partial_solution| partial_solution.is_satisfiable(self.test_value)),
            );
        }

        false
    }
}

struct PartialSolution<'a> {
    current_value: Value,
    remaining_operands: &'a [Value],
}

impl<'a> PartialSolution<'a> {
    fn is_complete(&self) -> bool {
        self.remaining_operands.is_empty()
    }

    fn is_satisfiable(&self, test_value: Value) -> bool {
        self.current_value <= test_value
    }
}

impl From<&str> for Equation {
    fn from(line: &str) -> Self {
        let (raw_test_value, raw_operands) = line.split_once(": ").unwrap();
        let test_value = raw_test_value.parse().unwrap();
        let operands = raw_operands
            .split_whitespace()
            .map(|operand| operand.parse().unwrap())
            .collect();
        Equation {
            test_value,
            operands,
        }
    }
}
