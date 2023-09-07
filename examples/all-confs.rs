use cfptime::CFPTime;

#[tokio::main]
async fn main() {
    let cfptime = CFPTime::new();

    let confs = match cfptime.get_confs().await {
        Ok(cfps) => cfps,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        },
    };

    println!("{:?}", confs);
}