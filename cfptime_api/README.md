# cfptime.rs
Library for interacting with CFPTime API - [api.cfptime.org/api/docs](https://api.cfptime.org/api/docs).

## Implemented Features
| Method | Resource |
|:------:|:---------|
| GET | /api/cfps/ |
| GET | /api/cfps/{id} |
| GET | /api/upcoming/ | 

## Example
```rust
use cfptime_api::{CFPTime, Conf};

#[tokio::main]
async fn main() {
    let cfptime = CFPTime::new();

    let confs: Vec<Conf> = cfptime.get_cfps().await.unwrap();
    for conf in confs.clone().iter() {
        println!(
            "#{:?} - Name: {:?}, Country: {:?}, Website: {:?}",
            conf.id,
            conf.name,
            conf.country,
            conf.website
        );
    }
}
```
