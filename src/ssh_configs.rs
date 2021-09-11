use regex::Regex;
use std::fs::File;
use std::io::Read;

fn extract_host_names(config_lines: Vec<String>) -> Vec<String> {
    let separator = Regex::new(r"[ \t]+").unwrap();
    let v= config_lines;
    v.into_iter()
        .map(|x| String::from(separator.split(&x).into_iter().collect::<Vec<&str>>()[1].trim()))
        .collect::<Vec<String>>()
}

pub fn read_config_lines() -> Vec<String> {
    let mut home_dir = dirs::home_dir().expect("Cannot get user home directory!");
    home_dir.push(".ssh");
    home_dir.push("config");
    let display = home_dir.display();

    let mut file = match File::open(&home_dir) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    }
    let configs = s.split(&['\n', '\r'][..])
        .filter(|x| x.starts_with("Host "))
        .map(|x| String::from(x))
        .collect::<Vec<String>>();
    if configs.len() == 0 {
        println!("No configured hosts in {}", display);
        std::process::exit(1);
    }
    return extract_host_names(configs);
}
