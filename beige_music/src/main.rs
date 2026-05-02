#[allow(dead_code)]
mod wave;
mod waveops;
mod wavetree;

use wave::Wave;
use waveops::stretch;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let infile = &args[1];
    let outfile = &args[2];

    let w = Wave::new_from_sndfile(infile);
    let s = stretch(&w, 0, w.len() / 4, 1.5);
    s.write_to_wav16(outfile);
}
