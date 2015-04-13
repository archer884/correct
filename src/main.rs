extern crate clap;
extern crate num;
extern crate strsim;
mod col;
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

    for pair in sim::best_matches(&params.dict, &params.word, 5) {
        println!("{:?}", pair);
    }
}
