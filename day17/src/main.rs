use regex::Regex;

#[derive(Clone, Debug)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u8>,
    pointer: usize,
    output: Vec<u8>,
}

impl Computer {
    fn step(&mut self) -> bool {
        let instruction = self.program[self.pointer];
        let literal_operand = || u64::from(self.program[self.pointer + 1]);
        let combo_operand = || match self.program[self.pointer + 1] {
            n @ 0..=3 => n.into(),
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => unreachable!(),
        };

        let mut jumped = false;

        match instruction {
            0 => self.register_a /= 2u64.pow(combo_operand().try_into().unwrap()),
            1 => {
                self.register_b ^= literal_operand();
            }
            2 => {
                self.register_b = combo_operand() % 8;
            }
            3 => {
                if self.register_a != 0 {
                    self.pointer = literal_operand().try_into().unwrap();
                    jumped = true;
                }
            }
            4 => {
                self.register_b ^= self.register_c;
            }
            5 => {
                self.output.push((combo_operand() % 8).try_into().unwrap());
            }
            6 => {
                self.register_b = self.register_a / 2u64.pow(combo_operand().try_into().unwrap());
            }
            7 => {
                self.register_c = self.register_a / 2u64.pow(combo_operand().try_into().unwrap());
            }
            _ => unreachable!(),
        }

        if !jumped {
            self.pointer += 2;
        }

        self.pointer < self.program.len()
    }
}

fn parse() -> Computer {
    let register_re = Regex::new(r"^Register [ABC]: (\d+)$").unwrap();
    let program_re = Regex::new(r"^Program: ((?:\d,)+\d)$").unwrap();

    let mut lines = std::io::stdin().lines().map(|line| line.unwrap());
    let register_a = register_re
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let register_b = register_re
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let register_c = register_re
        .captures(&lines.next().unwrap())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let program_line = lines.nth(1).unwrap();
    let program_str = program_re
        .captures(&program_line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let program = program_str
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect();

    Computer {
        register_a,
        register_b,
        register_c,
        program,
        pointer: 0,
        output: Vec::new(),
    }
}

fn part1(computer: &mut Computer) {
    while computer.step() {}
    println!(
        "{}",
        computer
            .output
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

fn part2(computer: &Computer) {
    // Here is a pseudocode listing of the program:
    //
    // start:
    // 1: B = (A % 8) ^ 2  // B is now a 3-bit value
    // 2: C = A >> B       // same as A // (2 ** B) where ** is exponentiation
    //                        and // is integer division
    // 3: B = B ^ C        // C might as well be (C % 8) here as only the lowest 3 bits
    //                        are used later
    // 4: A = A >> 3       // same as A // 8
    // 5: B = B ^ 7
    // 6: output(B % 8)
    // 7: goto start if A != 0
    //
    // Note that the program always loops through its entirety,
    // that output() is called once per loop,
    // and that A is only modified by a right shift by 3 bits.
    // Because the required output has 16 values, A must be zero on line 7 of the 16th loop,
    // and non-zero on previous loops. This means A must be a 48-bit value (well, 46 to 48).
    //
    // As A must be a 3-bit value at the start of the 16th loop, we can find the highest 3 bits
    // of the initial value of register A that produce the last output value.
    // After that, we can find the highest 6 bits of initial A that produce the second-to-last
    // output, then the highest 9 bits, and so on.
    //
    // This solution does obviously not work for all possible inputs, if such a thing is even
    // feasible; however, it might work for all puzzle inputs given out; they only seem to vary in
    // inconsequential instruction reorderings and the unused operand of opcode 4.

    let mut a: u64 = 0;

    for &output in computer.program.iter().rev() {
        for a_bits in 0..8 {
            // Check if having `a_bits` be the next highest 3 bits of A produces the wanted output.
            // We want the lowest possible value, so break as soon as a match is found.

            let a_candidate = (a << 3) | a_bits;
            let mut b = (a_candidate % 8) ^ 2;
            let c = a_candidate >> b;
            b ^= c;
            b ^= 7;
            if b % 8 == output.into() {
                a = a_candidate;
                break;
            }

            if a_bits == 7 {
                unreachable!("should have found a solution");
            }
        }
    }

    println!("{a}");
}

fn main() {
    let computer = parse();
    part1(&mut computer.clone());
    part2(&computer);
}
