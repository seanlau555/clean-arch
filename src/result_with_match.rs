#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: String) -> Result<Self, String> {
        if age >= 21 {
            Ok(Self { age, name: name })
        } else {
            Err("Age must be at least 21".to_owned())
        }
    }
}

fn main() {
    let child = Adult::new(15, "John".to_owned());
    let adult = Adult::new(21, "Sanjay".to_owned());
    match child {
        Ok(people) => println!("{} is {} years old", people.name, people.age),
        Err(error) => println!("Error: {}", error),
    }

    match adult {
        Ok(people) => println!("{} is {} years old", people.name, people.age),
        Err(error) => println!("Error: {}", error),
    }
}
