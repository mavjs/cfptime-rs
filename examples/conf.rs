use cfptime::CFPTime;

#[tokio::main]
async fn main() {
    let cfptime = CFPTime::new();

    let conf = match cfptime.get_conf(1735).await {
        Ok(cfp) => cfp,
        Err(error) => {
            eprintln!("Error: {:?}", error);
            std::process::exit(1);
        },
    };

    println!("{:?}", conf);
}