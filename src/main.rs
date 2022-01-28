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

fn main() {
    // Load in the codes from the file and store in a vector called 'commands'
    let commands = load_from_file_commands("src/data.txt");
    
    let ogr = find_oxygen_rating(commands.clone());
    println!("Oxygen Rating: {}", ogr);

    let csr = find_co2_rating(commands);
    println!("CO2 Rating: {}", csr);

    println!("Life support rating is: {}", ogr * csr);
}

fn find_oxygen_rating(mut oxy_commands: Vec<String>) -> i32 {
    println!("\nOXY_calculations");
    let mut index = 0;
    let mut counter = 0;
    let mut bit_flag = 0;

    while oxy_commands.len() > 1 {
        // First we need to know the most significant number for the position we're interested in
        for element in &oxy_commands {
            if element.chars().nth(index).unwrap() == '1' {
                counter += 1;
            }
        }
        println!("Oxy_Commands length: {:?}", oxy_commands.len());
        println!("Index/Counter: {}/{}", index, counter);

        if oxy_commands.len() == counter * 2 {
            println!("Equal, favor 1");
            bit_flag = 1;
        } else if counter > oxy_commands.len()/2 {
            bit_flag = 1;
        } else {
            bit_flag = 0;
        }
        counter = 0;
        println!("Bit Flag: {}\n", bit_flag);

        // Now we need to create a temp vector to hold only the codes we want to keep
        let mut temp = Vec::new();
        for element in oxy_commands {
            if element.chars().nth(index).unwrap() == bit_flag.to_string().chars().nth(0).unwrap() {
                temp.push(element);
            }
        }
        oxy_commands = temp;
        // println!("{:?}\n", oxy_commands);
        index += 1;
    }
    let result = &oxy_commands[0];
    println!("{:?}", result);
    i32::from_str_radix(result, 2).unwrap()
}

fn find_co2_rating(mut co2_commands: Vec<String>) -> i32 {
    println!("\nCO2_calculations");
    let mut index = 0;
    let mut counter = 0;
    let mut bit_flag = 0;

    while co2_commands.len() > 1 {
        // First we need to know the most significant number for the position we're interested in
        for element in &co2_commands {
            if element.chars().nth(index).unwrap() == '1' {
                counter += 1;
            }
        }
        println!("Index/Counter: {}/{}", index, counter);
        println!("Oxy_Commands length: {:?}", co2_commands.len());

        if co2_commands.len() == counter*2 {
            bit_flag = 0;
        } else if counter <= co2_commands.len()/2 {
            bit_flag = 1;
        } else {
            bit_flag = 0;
        }
        counter = 0;
        println!("Bit Flag: {}\n", bit_flag);

        // Now we need to create a temp vector to hold only the codes we want to keep
        let mut temp = Vec::new();
        for element in co2_commands {
            if element.chars().nth(index).unwrap() == bit_flag.to_string().chars().nth(0).unwrap() {
                temp.push(element);
            }
        }
        co2_commands = temp;
        // println!("{:?}\n", co2_commands);
        index += 1;
    }
    let result = &co2_commands[0];
    println!("{:?}", result);
    i32::from_str_radix(result, 2).unwrap()
}
