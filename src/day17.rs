use std::str::FromStr;

pub struct Computer {
    ra: u64,
    rb: u64,
    rc: u64,
    ip: usize,
    program: Vec<Opcode>,
    stdout: Vec<u64>,
}

type Opcode = u8;

impl Computer {
    fn new(ra: u64, rb: u64, rc: u64, program: Vec<Opcode>) -> Self {
        Self {
            ra,
            rb,
            rc,
            ip: 0,
            program,
            stdout: vec![],
        }
    }

    pub fn run(&mut self) -> String {
        while self.ip < self.program.len() {
            self.run_instruction();
        }

        let as_strings: Vec<String> = self
            .stdout
            .iter()
            .map(|value| format!("{}", value))
            .collect();

        as_strings.join(",")
    }

    /// Calculates the minimum value register a should be initialized with
    /// so that the program becomes a quine, outputting its own code.
    pub fn minimum_quine(&self) -> u64 {
        let mut ra = 0;

        // For my specific input, the program is equivalent to
        //
        // while a != 0 {
        //   b = a & 0b111
        //   b = b ^ 0b010
        //   c = a >> b
        //   a = a >> 3
        //   b = b ^ c
        //   b = b ^ 0b111
        //   out <- b & 0b111
        // }
        //
        // We can note a few things from this:
        //
        //  1. The loop is completely determined by a, which is shifted 3 bits to the right every iteration.
        //  2. The output is determined by the final value of b in each iteration.
        //    2.1. This is actually a function of the initial value of a at each iteration.
        //  3. c is completely irrelevant since it's only used as an auxiliary variable within each iteration.
        //
        // Assuming we know the final value of a after an iteration and the value that we output on that iteration,
        // we can determine which value a must have held right before the current one to produce that output.
        // This relies on the function described in 2.1 being a bijection (I think, I haven't actually proved this).
        //
        // Working backwards from the end, we can start by setting the value of a to 0 and backtrack with the expected
        // output at each iteration to find the initial value of a at the start of the execution.
        for opcode in self.program.iter().rev() {
            ra = Self::previous_ra(ra, *opcode as u64);
        }

        ra
    }

    fn previous_ra(ra: u64, target_output: u64) -> u64 {
        // We know after the current iteration, the value of a was ra.
        //
        // The initial value for a before the iteration, let's call it a_0,
        // must verify a_0 >> 3 == ra, or a_0 = ra || ra_, with ra_ a 3-bit value.
        //
        // On the other hand, the output value for each of those _candidate_ a_0
        // can be calculated and matched against the expected one.
        for ra_ in 0..8 {
            // 0..8 includes all possible 3-bit values, from 0b000 to 0b111
            let candidate = ra << 3 | ra_; // This is a fancy, bitwise version of 8 * ra + ra_, or ra || ra_

            // The final value of b after all the steps in the iteration
            let b = ra_ ^ 0b010 ^ (candidate >> (ra_ ^ 0b010)) ^ 0b111;
            let output = b & 0b111;

            if output == target_output {
                return candidate;
            }
        }

        panic!("No previous ra found");
    }

    fn run_instruction(&mut self) {
        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];
        match opcode {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Invalid opcode"),
        }

        if opcode != 3 {
            self.ip += 2;
        }
    }

    fn as_combo(&self, operand: u8) -> u64 {
        match operand {
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            7 => panic!("Invalid operand"),
            x => x as u64,
        }
    }

    fn adv(&mut self, operand: u8) {
        self.ra = self.ra / (1 << self.as_combo(operand));
    }

    fn bxl(&mut self, operand: u8) {
        self.rb = self.rb ^ operand as u64;
    }

    fn bst(&mut self, operand: u8) {
        self.rb = self.as_combo(operand) & 0b111;
    }

    fn jnz(&mut self, operand: u8) {
        if self.ra == 0 {
            self.ip += 2;
            return;
        }

        self.ip = operand as usize;
    }

    fn bxc(&mut self, _: u8) {
        self.rb = self.rb ^ self.rc;
    }

    fn out(&mut self, operand: u8) {
        self.stdout.push(self.as_combo(operand) & 0b111);
    }

    fn bdv(&mut self, operand: u8) {
        self.rb = self.ra / (1 << self.as_combo(operand));
    }

    fn cdv(&mut self, operand: u8) {
        self.rc = self.ra / (1 << self.as_combo(operand));
    }
}

impl FromStr for Computer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers_block, program_block) = s.split_once("\n\n").unwrap();
        let registers = registers_block
            .lines()
            .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
            .collect::<Vec<u64>>();
        let program = program_block
            .trim()
            .split_once(": ")
            .unwrap()
            .1
            .split(',')
            .map(|opcode| opcode.parse().unwrap())
            .collect::<Vec<Opcode>>();

        Ok(Computer::new(
            registers[0],
            registers[1],
            registers[2],
            program,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn run() {
        let mut computer = Computer::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", computer.run());
    }

    #[test]
    fn minimum_quine() {
        // Cannot use the example input because the logic is hardcoded for the actual input
        let input = fs::read_to_string("data/day17/input").unwrap();
        let computer: Computer = input.parse().unwrap();

        assert_eq!(190384113204239, computer.minimum_quine());

        let as_strings: Vec<String> = computer
            .program
            .iter()
            .map(|value| format!("{}", value))
            .collect();
        let mut test_computer = Computer {
            ra: computer.minimum_quine(),
            ..computer
        };
        assert_eq!(as_strings.join(","), test_computer.run());
    }
}
