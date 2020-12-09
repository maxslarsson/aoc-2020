use std::collections::HashMap;

pub fn part1(input: String) {
    let mut instructions: Vec<(&str, isize)> = Vec::new();

    for instru in input.split("\n") {
        let split: Vec<_> = instru.split(" ").collect();

        assert_eq!(split.len(), 2);

        instructions.push((split[0], split[1].parse().unwrap()));
    }

    let mut index: isize = 0;
    let mut accumulator = 0;

    let mut visited_indexes = Vec::new();

    while !visited_indexes.contains(&index) {
        visited_indexes.push(index);

        let instruction = instructions[index as usize];

        match instruction.0 {
            "nop" => (),
            "acc" => accumulator += instruction.1,
            "jmp" => index += instruction.1,
            _ => panic!("Unknown instruction!")
        }

        if instruction.0 != "jmp" {
            index += 1;
        }
    }

    println!("{}", accumulator);
}

pub fn part2(input: String) {
    let mut instructions: Vec<(&str, isize)> = Vec::new();

    for instru in input.split("\n") {
        let split: Vec<_> = instru.split(" ").collect();

        assert_eq!(split.len(), 2);
        
        instructions.push((split[0], split[1].parse().unwrap()));
    }
    
    let index = 0;
    let accumulator = 0;
    
    let visited_indexes = HashMap::new();

    println!("{}", recursive(index, accumulator, visited_indexes, &instructions, false));

    // while (index as usize) < instructions.len() {
    //     let mut went_backwards = false; 
    //     if visited_indexes.contains(&index) {
    //         went_backwards = true;
    //         loop {
    //             index = visited_indexes.pop().unwrap();
                
    //             match instructions[index as usize].0 {
    //                 "nop" | "jmp" => break,
    //                 _ => ()
    //             }
    //         }
    //     }

    //     visited_indexes.push(index);
        
    //     let instruction = instructions[index as usize];
        
    //     if went_backwards {
    //         match instruction.0 {
    //             "nop" => index += instruction.1,
    //             "jmp" => (),
    //             "acc" => accumulator += instruction.1,
    //             _ => panic!("Unknown instruction!")
    //         }
    //     } else {
    //         match instruction.0 {
    //             "nop" => (),
    //             "jmp" => index += instruction.1,
    //             "acc" => accumulator += instruction.1,
    //             _ => panic!("Unknown instruction!")
    //         }
    //     }

    //     if instruction.0 != "jmp" {
    //         index += 1;
    //     }
    // }

}

fn recursive(index: isize, accumulator: isize, mut visited_indexes: HashMap<isize, usize>, instructions: &Vec<(&str, isize)>, changed: bool) -> isize {
    if (index as usize) == instructions.len() {
        return accumulator;
    }

    if let Some(count) = visited_indexes.get_mut(&index) {
        if *count < 1 {
            *count += 1;
        } else {
            return 0;
        }
    } else {
        visited_indexes.insert(index, 1).unwrap_none();
    }

    let instruction = instructions[index as usize];

    match instruction.0 {
        "nop" => {
            if changed {
                return recursive(index + 1, accumulator, visited_indexes.clone(), instructions, changed) 
            } else {
                return recursive(index + 1, accumulator, visited_indexes.clone(), instructions, changed) + recursive(index + instruction.1, accumulator, visited_indexes.clone(), instructions, !changed)
            }
        },
        "jmp" => {
            if changed {
                return recursive(index + instruction.1, accumulator, visited_indexes.clone(), instructions, changed)
            } else {
                return recursive(index + instruction.1, accumulator, visited_indexes.clone(), instructions, changed) + recursive(index + 1, accumulator, visited_indexes.clone(), instructions, !changed) 
            }
        },
        "acc" => return recursive(index + 1, accumulator + instruction.1, visited_indexes.clone(), instructions, changed),
        _ => panic!("Unknown instruction!")
    }

}