use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("data/day17/input").unwrap();
    let mut computer: Computer = input.parse().unwrap();
    let stdout = computer.run();
    println!("{}", stdout);
}

struct Computer {
    ra: u64,
    rb: u64,
    rc: u64,
    ip: usize,
    program: Vec<Opcode>,
    stdout: Vec<u64>,
}

type Opcode = u8;

impl Computer {
    fn run(&mut self) -> String {
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

        Ok(Computer {
            ra: registers[0],
            rb: registers[1],
            rc: registers[2],
            ip: 0,
            program,
            stdout: vec![],
        })
    }
}
