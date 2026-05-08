#[allow(dead_code)]
mod wave;
mod waveops;
mod wavetree;

use wave::Wave;
use waveops::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let infile = &args[1];
    let outfile = &args[2];

    // read from src
    let src = Wave::new_from_sndfile(infile);
    println!("src len = {}", src.len());

    // write to out
    let out = stretch(&src, 0, src.len() - 1, 2.0);

    out.write_to_wav16(outfile);
}
