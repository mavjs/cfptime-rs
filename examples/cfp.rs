use cfptime::CFPTime;

#[tokio::main]
async fn main() {
    let cfptime = CFPTime::new();

    let cfp = match cfptime.get_cfp(1729).await {
        Ok(cfp) => cfp,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        },
    };

    println!("{:?}", cfp);
}