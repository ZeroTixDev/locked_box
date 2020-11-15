use std::io;
struct Box {
    locked: bool,
    _content: Vec<String>,
}
impl Box {
    fn new() -> Box {
        Box {
            locked: false,
            _content: Vec::new(),
        }
    }
    fn remove(&mut self, value: &str) -> String {
        if !self.locked {
            let mut index = 0;
            let mut matched: bool = false;
            let mut selected: String = String::from("");
            for element in self._content.iter() {
                if element.to_string() == value.to_string() {
                    matched = true;
                    selected = element.to_string();
                    break;
                }
                index += 1;
            }
            if matched {
                self._content.remove(index);
                return format!("Successfully removed [{}] from the box", selected);
            }
            return format!("Error: Cannot find [{}] in the box", value);
        } else {
            return format!("Error: Cannot remove item from box because it is locked");
        }
    }
    fn view(&self) -> String {
        if self.locked {
            return String::from("Error: Cannot view content because box is locked");
        } else {
            let mut string: String = String::from("");
            for value in self._content.iter() {
                string.push_str(format!("[{}] ", &*value).as_str());
            }
            if string.as_str() == "" {
            	string = "[]".to_string();
            }
            return string;
        }
    }
    fn add(&mut self, value: &str) -> String {
    	if value.trim() == "" {
    		return format!("Error: Cannot add '' to box")
    	}
        if !self.locked {
        	for element in self._content.iter() {
        		if element.to_string() == value.to_string() {
        			return format!("Error: Cannot add [{}] to the box because it is already in the box", value.to_string())
        		}
        	}
            self._content.push(value.to_string());
            return format!("Succesfully added [{}] to the box", value.to_string());
        } else {
            return format!(
                "Error: Cannot add [{}] to the box because it is locked",
                value.to_string()
            );
        }
    }
    fn lock(&mut self) -> String {
        if !self.locked {
            self.locked = true;
            return format!("Successfully locked the box");
        } else {
            return format!("Error: Cannot lock the box because it is already locked");
        }
    }
    fn unlock(&mut self) -> String {
        if self.locked {
            self.locked = false;
            return format!("Successfully unlocked the box");
        } else {
            return format!("Error: Cannot unlock the box because it is already unlocked");
        }
    }
}
fn main() {
    let mut game_box = Box::new();
    println!(
        "There is a box. It is currently unlocked. Commands [add, lock, unlock, view, remove, help]"
    );
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let values: Vec<&str> = guess.split_whitespace().collect();
        if values.len() == 0 {
        	println!("Error: Operation cannot be done");
        	continue;
        }
       	if values[0] == "add" {
       		let mut on_first: bool = false;
       		let mut string = String::new();
       		for element in values.iter() {
       			if on_first {
       				string.push_str(element);
       				string.push_str(" ");
       			}
       			if !on_first {
       				on_first = true;
       			}
       		}
       		string.pop();
       		println!("{}", game_box.add(string.as_str()));
       	}else if values[0] == "lock" {
       		println!("{}", game_box.lock());
       		continue;
       	}else if values[0] == "help" {
       		println!("Commands [add, lock, unlock, view, remove, help]");
       		continue;
       	}else if values[0] == "unlock" {
       		println!("{}",game_box.unlock());
       		continue;
       	}else if values[0] == "view" {
       		println!("{}",game_box.view());
       		continue;
       	}else if values[0]== "remove" {
       		let mut on_first: bool = false;
       		let mut string = String::new();
       		for element in values.iter() {
       			if on_first {
       				string.push_str(element);
       				string.push_str(" ");
       			}
       			if !on_first {
       				on_first = true;
       			}
       		}
       		string.pop();
       		println!("{}",game_box.remove(string.as_str()));
       		continue;
       	}else {
       		println!("Error: Could not read command");
       		continue;
       	}
    }
}
