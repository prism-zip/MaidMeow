use std::fs;
use std::collections::HashMap;


pub fn filter(line: &str) -> String {

    let mut formated = "".to_string();
    for symbol in line.chars() {
        if symbol == '"' {
            formated = format!("{}{}", formated, "");
        } else if symbol == '\\' {
            formated = format!("{}{}", formated, "\\\\");
        } else if symbol == '#' {
            formated = format!("{}{}", formated, "");
        } else {
            formated = format!("{}{}", formated, symbol);
        }
    }

    return formated;

}

pub fn read_meow(path: &str, debug: bool) -> HashMap<String, String> {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let mut result = HashMap::new();

    loop {
        match lines.next() {
            Some(line) => {
                let sys_args: Vec<String> = line.split_whitespace().map(String::from).collect();
                
                if debug == true {
                    println!("{:?}", sys_args);
                }

                if sys_args.len() == 4 {
                    let key = filter(&sys_args[1]); 
                    let mut value = "".to_string();
                    for item in &sys_args[3..] {
                        value = format!("{}{}", value, item);
                    }
                    let val = filter(&value);
                    result.insert(key, val);  
                }

            }
            None => break,
        }
    }
    
    return result;
    

}
