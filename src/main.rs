extern crate clap;
extern crate strsim;
mod params;
mod sim;

fn main() {
    // Grab args, handle possible early return
    let params = match params::read_args() {
        Ok(params) => params,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match sim::best_match(&params.dict, &params.word) {
        Some(word) => println!("{}", word),
        None => println!("No suggestions"),
    }
}
