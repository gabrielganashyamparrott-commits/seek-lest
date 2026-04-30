mod wave;
use wave::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let infile = &args[1];
    let outfile = &args[2];

    let mut w = Wave::new_from_sndfile(infile);
    let samplerate = w.samplerate();
    let channels = w.channels();
    let _ew = Wave::new(samplerate, channels);

    println!("samplerate = {}", w.samplerate());
    println!("channels = {}", w.channels());

    println!("testing functions");
    let _ = w.get(0);
    if w.channels() < 2 {
        w.push(Sample::Mono(0.0));
        w.safe_set(Sample::Mono(0.0), 0);
        w.set(Sample::Mono(0.0), 0);
    } else {
        w.push(Sample::Stereo {left: 0.0, right: 0.0});
        w.safe_set(Sample::Stereo {left: 0.0, right: 0.0}, 0);
        w.set(Sample::Stereo {left: 0.0, right: 0.0}, 0);
    }
    println!("tests passed");

    w.write_to_wav16(outfile);
    println!("copied to {} successfully", outfile);
}
