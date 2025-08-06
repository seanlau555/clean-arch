struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey {
        q1: None,
        q2: Some(false),
        q3: Some(String::from("A")),
    };

    match response.q1 {
        Some(ans) => println!("q1: {:?}", ans),
        None => println!("q1: no response"),
    }

    match response.q2 {
        Some(ans) => println!("q2: {:?}", ans),
        None => println!("q1: no response"),
    }

    match response.q3 {
        Some(ans) => println!("q3: {:?}", ans),
        None => println!("q1: no response"),
    }
}
