#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err(String::from("Invalid choice")),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<MenuChoice, String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(choice)
}

fn main() {
    let choice = pick_choice("end");
    println!("choice value = {:?}", choice);
}
