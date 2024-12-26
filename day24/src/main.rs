use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

type Inputs = HashMap<String, bool>;
type Gates = HashMap<((String, String), Op), String>;

fn parse() -> (Inputs, Gates) {
    let mut inputs = HashMap::new();
    let mut gates = HashMap::new();

    let input_re = Regex::new(r"^(?<name>\w+): (?<value>0|1)$").unwrap();
    let gate_re =
        Regex::new(r"^(?<input1>\w+) (?<op>AND|OR|XOR) (?<input2>\w+) -> (?<output>\w+)$").unwrap();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if let Some(captures) = input_re.captures(&line) {
            let name = captures.name("name").unwrap().as_str().to_string();
            let value = captures.name("value").unwrap().as_str() == "1";
            inputs.insert(name, value);
        } else if let Some(captures) = gate_re.captures(&line) {
            let input1 = captures.name("input1").unwrap().as_str().to_string();
            let input2 = captures.name("input2").unwrap().as_str().to_string();
            let op = match captures.name("op").unwrap().as_str() {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => unreachable!(),
            };
            let output = captures.name("output").unwrap().as_str().to_string();
            let inputs = if input1 < input2 {
                (input1, input2)
            } else {
                (input2, input1)
            };
            gates.insert((inputs, op), output);
        }
    }

    (inputs, gates)
}

fn part1(mut inputs: Inputs, gates: &Gates) {
    loop {
        let mut done = true;

        for (((input1, input2), op), output) in gates {
            if inputs.contains_key(output) {
                continue;
            }

            if let (Some(input_value1), Some(input_value2)) =
                (inputs.get(input1), inputs.get(input2))
            {
                let output_value = match op {
                    Op::And => input_value1 & input_value2,
                    Op::Or => input_value1 | input_value2,
                    Op::Xor => input_value1 ^ input_value2,
                };
                inputs.insert(output.to_string(), output_value);
                done = false;
            }
        }

        if done {
            break;
        }
    }

    let mut z_keys = inputs
        .keys()
        .filter(|key| key.starts_with('z'))
        .collect::<Vec<_>>();
    z_keys.sort();
    let mut output: u64 = 0;
    for (i, key) in z_keys.into_iter().enumerate() {
        if *inputs.get(key).unwrap() {
            output |= 1 << i;
        }
    }

    println!("{output}");
}

fn main() {
    let (inputs, gates) = parse();
    part1(inputs, &gates);
}
