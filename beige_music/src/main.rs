#[allow(dead_code)]
mod wave;
mod waveops;
use wave::Wave;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let infile = &args[1];
    let outfile = &args[2];

    let w = Wave::new_from_sndfile(infile);
    w.write_to_wav16(outfile);
}
