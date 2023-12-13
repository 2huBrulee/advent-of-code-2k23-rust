use crate::core::FileLines;
use std::collections::HashMap;

type Instructions = Vec<String>;

#[derive(Debug)]
struct NodeDef {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct Input {
    instructions: Instructions,
    defs: HashMap<String, NodeDef>,
}

pub fn solve(lines: FileLines) -> i128 {
    let input = parse_input(lines);

    // println!("{:?}", input);

    let initial_nodes = input
        .defs
        .keys()
        .filter(|n| n.ends_with("A"))
        .collect::<Vec<_>>();

    println!("{:?}", initial_nodes);

    let steps_to_z_vec = initial_nodes
        .clone()
        .iter()
        .map(|initial_node| {
            let mut steps = 0_i32;
            let mut z_positions: Vec<i32> = Vec::new();
            let mut visited_nodes: Vec<String> = Vec::new();
            let mut current_node = (*initial_node).clone();

            loop {
                steps += 1;

                let instruction_index: usize =
                    ((steps - 1) % input.instructions.len() as i32) as usize;

                let current_instruction = input.instructions.get(instruction_index).unwrap();

                let current_node_def = input.defs.get(&current_node).unwrap();

                match current_instruction.as_str() {
                    "L" => {
                        current_node = current_node_def.left.clone();
                        visited_nodes.push(current_node_def.left.clone());
                    }
                    "R" => {
                        current_node = current_node_def.right.clone();
                        visited_nodes.push(current_node_def.right.clone());
                    }
                    _ => panic!("bad input"),
                }

                if current_node.ends_with("Z") {
                    z_positions.push(steps);
                    break;
                }

                // TODO check for cycles
            }

            println!("{:?}", z_positions);
            println!("{:?}", visited_nodes);

            z_positions.get(0).unwrap().clone()
        })
        .collect::<Vec<_>>();

    let mut total_steps = 1_i128;

    for steps_to_z in steps_to_z_vec {
        total_steps = num::integer::lcm(total_steps, steps_to_z as i128);
    }

    total_steps
}

fn parse_input(lines: FileLines) -> Input {
    let mut input = Input {
        instructions: Vec::new(),
        defs: HashMap::new(),
    };

    let lines_vec = lines.map(|l| l.unwrap()).collect::<Vec<_>>();

    let (instructions_line, other_lines) = lines_vec.split_first().unwrap();

    let instructions = instructions_line
        .split("")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    input.instructions = instructions;

    let defs_lines = other_lines.iter().filter(|l| !l.is_empty());

    for def_line in defs_lines {
        let (name, rest) = def_line.split_once(" = ").unwrap();

        let replaced_string = rest.replace("(", "").replace(")", "").replace(" ", "");

        let (left, right) = replaced_string.split_once(",").unwrap();

        let node_def = NodeDef {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        };

        input.defs.insert(node_def.name.clone(), node_def);
    }

    input
}
