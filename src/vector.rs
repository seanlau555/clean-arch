struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            name: String::from("John"),
            fav_color: String::from("Blue"),
            age: 7,
        },
        Person {
            name: String::from("Anna"),
            fav_color: String::from("Purple"),
            age: 9,
        },
        Person {
            name: String::from("Katie"),
            fav_color: String::from("Green"),
            age: 14,
        },
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
    }
