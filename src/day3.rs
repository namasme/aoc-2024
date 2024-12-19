use regex::Regex;

pub struct CorruptedProgram {
    instructions: Vec<Instruction>,
}

enum Instruction {
    Mul(Value, Value),
    Do,
    Dont,
}

type Value = u32;

impl CorruptedProgram {
    pub fn parse(input: &str) -> Self {
        let re =
            Regex::new(r"mul\((?<x>\d+),(?<y>\d+)\)|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();

        let instructions = re
            .captures_iter(input)
            .map(|cap| {
                if cap.name("do").is_some() {
                    return Instruction::Do;
                } else if cap.name("dont").is_some() {
                    return Instruction::Dont;
                } else {
                    let x = &cap[1];
                    let y = &cap[2];
                    return Instruction::Mul(x.parse().unwrap(), y.parse().unwrap());
                }
            })
            .collect();

        Self { instructions }
    }

    pub fn parse_first(input: &str) -> Self {
        let filtered_instructions = Self::parse(input)
            .instructions
            .into_iter()
            .filter(|instruction| {
                if let Instruction::Mul(_, _) = instruction {
                    return true;
                } else {
                    return false;
                }
            })
            .collect();

        Self {
            instructions: filtered_instructions,
        }
    }

    pub fn result(&self) -> Value {
        let mut acc = 0;
        let mut enabled = true;

        for instruction in &self.instructions {
            match instruction {
                Instruction::Mul(x, y) => {
                    if enabled {
                        acc += x * y;
                    }
                }
                Instruction::Do => {
                    enabled = true;
                }
                Instruction::Dont => {
                    enabled = false;
                }
            }
        }

        acc
    }
}
