enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(50.0, "Billy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, String::from("Amy")),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Backstage ticket Holder: {:?}, price: {:?}", holder, price)
            }
            Ticket::Standard(price) => println!("Standard ticket Price: {:?}", price),
            Ticket::Vip(price, holder) => {
                println!("VIP ticket Holder: {:?}, price: {:?}", holder, price)
            }
        }
    }
}
