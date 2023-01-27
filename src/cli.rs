
pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone(); // error when no param
    let name = "Joao";
    let status = "100%";
  
    if command == "hello" {
      println!("Hi {}, how are you?", name);
    } else if command == "status" {
      println!("Status is {}", status);
    } else {
      println!("That is not a valid command");
    }
  }