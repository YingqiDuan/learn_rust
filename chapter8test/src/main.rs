use std::collections::HashMap;
use std::io;

fn main() {
    {
        {
            let mut numbers = vec![3, 2, 1, 2, 3, 4, 5, 2, 3];

            let median = get_median(&mut numbers);

            let mode = get_mode(&numbers);

            println!("numbers is {:?}", numbers);
            println!("median is {:?}", median);
            println!("mode is {:?}", mode);
        }

        fn get_median(numbers: &mut Vec<i32>) -> i32 {
            numbers.sort();
            let mid = numbers.len() / 2;
            numbers[mid]
        }

        fn get_mode(numbers: &Vec<i32>) -> i32 {
            let mut map = HashMap::new();

            for &num in numbers {
                *map.entry(num).or_insert(0) += 1;
            }

            let mut mode = numbers[0];
            let mut max_count = 0;
            for (&key, &count) in &map {
                if count > max_count {
                    max_count = count;
                    mode = key;
                }
            }
            mode
        }
    }

    {
        {
            let sentence = "hello world first apple hay pig";

            let pig_latin_sentence = convert_to_pig_latin(sentence);
            println!("sentence is {:?}", sentence);
            println!("pig latin sentence is {:?}", pig_latin_sentence)
        }

        fn word_to_pig_latin(word: &str) -> String {
            let vowels = vec!['a', 'e', 'i', 'o', 'u'];
            if word.is_empty() {
                return String::new();
            }

            let first_char = word.chars().next().unwrap();
            let first_char_lower = first_char.to_ascii_lowercase();

            if vowels.contains(&first_char_lower) {
                format!("{}-hay", word)
            } else {
                let mut chars = word.chars();
                chars.next();
                let rest: String = chars.collect();
                format!("{}-{}ay", rest, first_char.to_lowercase())
            }
        }

        fn convert_to_pig_latin(sentence: &str) -> String {
            let words: Vec<&str> = sentence.split_whitespace().collect();

            let converted_words: Vec<String> = words.iter().map(|w| word_to_pig_latin(w)).collect();

            converted_words.join(" ")
        }
    }

    {
        {
            let mut company: HashMap<String, Vec<String>> = HashMap::new();

            loop {
                println!("\nEnter your commands:");
                println!("1) Add name to department");
                println!("2) List department");
                println!("3) List All");
                println!("4) Quit");

                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_err() {
                    println!("Failed to read input.");
                    continue;
                }

                let input = input.trim();
                if input.eq_ignore_ascii_case("Quit") || input == "4" {
                    println!("Quitting.");
                    break;
                }

                if input.starts_with("Add ") || input.starts_with("add ") {
                    let parts: Vec<&str> = input.split_whitespace().collect();
                    if parts.len() < 4 {
                        println!("Invalid input. Please use: Add <name> to <department>");
                        continue;
                    }
                    let name = parts[1];
                    let department = parts[3];

                    company
                        .entry(department.to_string())
                        .or_insert(Vec::new())
                        .push(name.to_string());
                } else if input.starts_with("List ") || input.starts_with("list ") {
                    let parts: Vec<&str> = input.split_whitespace().collect();
                    if parts.len() < 2 {
                        println!("Invalid input. Please use: List <department> or List All");
                        continue;
                    }
                    let cmd = parts[1];
                    if cmd.eq_ignore_ascii_case("All") {
                        list_all(&company);
                    } else {
                        list_department(&company, cmd);
                    }
                } else {
                    println!("Invalid input.");
                }
            }
        }
        fn list_department(company: &HashMap<String, Vec<String>>, department: &str) {
            match company.get(department) {
                Some(employees) => {
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    println!("department {} employees", department);
                    for emp in sorted_employees {
                        println!(" - {}", emp)
                    }
                }
                None => println!("department {} employees not found", department),
            }
        }

        fn list_all(company: &HashMap<String, Vec<String>>) {
            let mut departments: Vec<_> = company.keys().collect();
            departments.sort();

            for dept in departments {
                list_department(company, dept);
            }
        }
    }
}
