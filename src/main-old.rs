use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file_commands(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let code: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    code
}

fn which_one (list: &Vec<String>, position: usize) -> char {
    let mut counter = 0;
    for item in list {
        if item.chars().nth(position).unwrap() == '1' {
            counter += 1;
        }
    }
    if counter >= list.len()/2 {
        '1'
    } else {
        '0'
    }
}

fn diag2(codes: Vec<String>) {
    let mut oxy: Vec<String> = Vec::new();
    let mut co2 = codes.clone();
    let lines = &codes.len();
    let length = codes[0].len();
    let mut result = vec![0; length];
    let mut need = '0';
    let mut index = 0;
    for code in &codes {
        for c in code.chars().enumerate() {
            if c.1 == '1' {
                result[c.0] += 1;
            }
        }
    }
    println!("{:?}", result);
    
    // Let's get the first set of numbers into oxy vector, then loop over oxy to narrow it down to a single number
    need = which_one(&codes.clone(), 0);

    for i in 0 .. codes.len() {
        // println!("{}", codes[i]);
        println!("Need: {}, Codes_Char: {}", need, codes[i].chars().nth(index).unwrap());
        if codes[i].chars().nth(index).unwrap() == need {
            oxy.push(codes[i]);
        }
    }
    println!("Oxy: {:?}\n", oxy);
    index += 1;
/* 
    while oxy.len() != 1 && index < result.len() {
        println!("**Entering While loop with char pos: {}", index+1);
        need = which_one(&oxy, index);
        // if result[index] >= lines/2 {
        //     need = '1';
        // } else {
        //     need = '0';
        // }
        let new_oxy = oxy.clone();
        for i in 0 .. new_oxy.len() {
            println!("Entering loop with index {} of {}", i, new_oxy.len()-1);
            println!("Need: {}, Oxy_Char: {}", need, new_oxy[i].chars().nth(index).unwrap());
            if new_oxy[i].chars().nth(index).unwrap() != need {
                let position = oxy.iter().position(|&x| x == new_oxy[i]).unwrap();
                oxy.remove(position);
                println!("Removing {}", new_oxy[i]);
            }
            println!("New_Oxy: {:?}", new_oxy);
            println!("Oxy: {:?}\n", oxy);
        }
        index += 1;
        println!("Oxygen Generator rating: {:?}", isize::from_str_radix(&oxy[0], 2).unwrap());
    }

    //     index += 1;
    //     println!("oxy len: {}", oxy.len());
    //     println!("oxy: {:?}", oxy);
    //     let mut results = oxy.clone();
    // }
    // println!("{:?}", oxy);
    */
}

fn main() {
    let commands = load_from_file_commands("src/test.txt");
    diag2(commands)
}
