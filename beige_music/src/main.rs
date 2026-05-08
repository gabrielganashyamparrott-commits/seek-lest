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

    let src = Wave::new_from_sndfile(infile);
    let mut out = Wave::new(src.samplerate(), src.channels());
    out.write_to_wav16(outfile);
}
