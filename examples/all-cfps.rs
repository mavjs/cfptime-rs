use cfptime::CFPTime;

#[tokio::main]
async fn main() {
    let cfptime = CFPTime::new();

    let cfps = match cfptime.get_cfps().await {
        Ok(cfps) => cfps,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        },
    };

    println!("{:?}", cfps);
}