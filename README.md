# cfptime.rs
Library for cfptime.org API

## Implemented Features
| Method | Resource |
|:------:|:---------|
| GET | /api/cfps/ |
| GET | /api/cfps/{id} |

## Example
```rust
extern crate cfptime;

use cfptime::*;

fn main() -> Result<(), CfpError> {
    let cfp = CfpTime::new();
    let conf: Conf = cfp.get_cfp_id(5i32)?;
    println!("{:#?}", conf);
    Ok(())
}
```
