use std::process::Command;

pub fn preprocess(input: &str) -> Option<String>{
    let output = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(input)
        .output();

    if let Ok(output) = output {
        if output.status.success(){
            return Some(String::from_utf8_lossy(&output.stdout).to_string());
        }
    }
        
    return None;
}