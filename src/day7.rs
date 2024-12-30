use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<Equation> {
    input.lines().map(|line| Equation::from(line)).collect()
}

pub fn compute_calibration_result(
    equations: &[Equation],
    allowed_operations: &HashSet<Operation>,
) -> Value {
    equations
        .iter()
        .filter(|equation| equation.is_satisfiable(allowed_operations))
        .map(|equation| equation.test_value)
        .sum()
}

type Value = u64;

#[derive(Eq, Hash, PartialEq)]
pub enum Operation {
    Add,
    Mul,
    Concat,
}

pub struct Equation {
    test_value: Value,
    operands: Vec<Value>,
}

impl Equation {
    fn is_satisfiable(&self, allowed_operations: &HashSet<Operation>) -> bool {
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
            let candidates = partial_solution.advance(&allowed_operations);

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
    fn advance(&self, allowed_operations: &HashSet<Operation>) -> Vec<Self> {
        allowed_operations
            .iter()
            .map(|operation| self.apply(operation))
            .collect()
    }

    fn apply(&self, operation: &Operation) -> Self {
        match operation {
            Operation::Add => self.next_add(),
            Operation::Mul => self.next_mul(),
            Operation::Concat => self.next_concat(),
        }
    }

    fn next_add(&self) -> Self {
        Self {
            current_value: self.current_value + self.remaining_operands[0],
            remaining_operands: &self.remaining_operands[1..],
        }
    }

    fn next_mul(&self) -> Self {
        Self {
            current_value: self.current_value * self.remaining_operands[0],
            remaining_operands: &self.remaining_operands[1..],
        }
    }

    fn next_concat(&self) -> Self {
        Self {
            current_value: Self::concat(self.current_value, self.remaining_operands[0]),
            remaining_operands: &self.remaining_operands[1..],
        }
    }

    fn concat(left: Value, right: Value) -> Value {
        format!("{}{}", left, right).parse().unwrap()
    }

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
