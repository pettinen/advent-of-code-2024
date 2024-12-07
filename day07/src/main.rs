fn parse() -> Vec<(u64, Vec<u64>)> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (result, operands) = line.split_once(": ").unwrap();
            let result = result.parse().unwrap();
            let operands = operands
                .split(' ')
                .map(|operand| operand.parse().unwrap())
                .collect();
            (result, operands)
        })
        .collect()
}

fn possible(result: u64, operands: &[u64]) -> bool {
    if operands.len() == 2 {
        return operands[0] + operands[1] == result || operands[0] * operands[1] == result;
    }
    let mut new_operands = Vec::with_capacity(operands.len() - 1);
    new_operands.push(operands[0] + operands[1]);
    new_operands.extend(&operands[2..]);
    if possible(result, &new_operands) {
        return true;
    }
    new_operands[0] = operands[0] * operands[1];
    possible(result, &new_operands)
}

fn part1(inputs: &[(u64, Vec<u64>)]) {
    let sum: u64 = inputs
        .iter()
        .filter_map(|(result, operands)| possible(*result, operands).then_some(result))
        .sum();
    println!("{sum}");
}

fn concat(a: u64, b: u64) -> u64 {
    let scale = b.ilog10() + 1;
    a * 10u64.pow(scale) + b
}

fn possible_with_concat(result: u64, operands: &[u64]) -> bool {
    if operands.len() == 2 {
        return operands[0] + operands[1] == result
            || operands[0] * operands[1] == result
            || concat(operands[0], operands[1]) == result;
    }
    let mut new_operands = Vec::with_capacity(operands.len() - 1);
    new_operands.push(operands[0] + operands[1]);
    new_operands.extend(&operands[2..]);
    if possible_with_concat(result, &new_operands) {
        return true;
    }
    new_operands[0] = operands[0] * operands[1];
    if possible_with_concat(result, &new_operands) {
        return true;
    }
    new_operands[0] = concat(operands[0], operands[1]);
    possible_with_concat(result, &new_operands)
}

fn part2(inputs: &[(u64, Vec<u64>)]) {
    let sum: u64 = inputs
        .iter()
        .filter_map(|(result, operands)| possible_with_concat(*result, operands).then_some(result))
        .sum();
    println!("{sum}");
}

fn main() {
    let inputs = parse();
    part1(&inputs);
    part2(&inputs);
}
