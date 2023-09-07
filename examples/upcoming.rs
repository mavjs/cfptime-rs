use cfptime::CFPTime;

#[tokio::main]
async fn main() {
    let cfptime = CFPTime::new();

    let upcomings = match cfptime.get_upcoming().await {
        Ok(cfps) => cfps,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        },
    };

    println!("{:?}", upcomings);
}