pub enum Channels {
 One,
 Two,
}

#[derive(Copy, Clone)]
pub enum Sample {
    Mono(f32),
    Stereo {left: f32, right: f32},
}

#[derive(Clone)]
pub struct Wave {
    samplerate: usize,
    channels: usize,
    samples: Vec<Sample>,
}

impl Wave {
    pub fn new(
        samplerate: usize,
        channels: usize,
    ) -> Wave {
        Wave {
            samplerate,
            channels,
            samples: Vec::new(),
        }
    }

    pub fn new_from_sndfile(path: &str) -> Wave {
        use sndfile::{
            OpenOptions,
            ReadOptions,
            SndFileIO,
        };

        let samplerate;
        let frames;
        let channels;
        let interleaved_vec: Vec<f32>;
        let mut samples: Vec<Sample> = Vec::new();

        // read sndfile to vec
        let mut sf = OpenOptions::ReadOnly(
            ReadOptions::Auto,
        ).from_path(path)
        .unwrap();

        // read metadata
        samplerate = sf.get_samplerate();
        frames = sf.len().unwrap() as usize;
        channels = sf.get_channels();

        // read sound file to Vec<f32>
        interleaved_vec = sf.read_all_to_vec().unwrap();

        // read vec to Wave
        if channels < 2 {
            for frame_num in 0..frames {
                let s = interleaved_vec[frame_num];
                let sample = Sample::Mono(s);
                samples.push(sample);
            }
        } else {
            for frame_num in 0..frames {
                let sl = interleaved_vec[frame_num*2];
                let sr = interleaved_vec[(frame_num*2) + 1];
                let sample = Sample::Stereo {left: sl, right: sr};
                samples.push(sample);
            }
        }
        
        // return Wave
        Wave {
            samplerate,
            channels,
            samples,
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        let mut out = Vec::new();
        for s in self.samples.iter() {
            match s {
                Sample::Mono(x) => {
                    out.push(*x);
                },
                Sample::Stereo {left: x, right: y} => {
                    out.push(*x);
                    out.push(*y);
                },
            }
        };

        out
    }

    pub fn write_to_wav16(&self, path: &str) {
        use sndfile::{
            OpenOptions,
            WriteOptions,
            MajorFormat,
            SubtypeFormat,
            Endian,
            SndFileIO,
        };

        // open sndfile for writing
        let mut sf = OpenOptions::WriteOnly(
            WriteOptions::new(
                MajorFormat::WAV,
                SubtypeFormat::PCM_16,
                Endian::File,
                self.samplerate,
                self.channels,
            ),
        ).from_path(path)
        .unwrap();

        // copy to vec
        let v = self.to_vec();

        // write to sndfile
        sf.write_from_slice(&v).unwrap();
    }

    pub fn samplerate(&self) -> usize {
        self.samplerate
    }
    pub fn channels(&self) -> usize {
        self.channels
    }
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    pub fn get(&self, frame: usize) -> Sample {
        self.samples[frame].clone()
    }
    pub fn push(&mut self, sample: Sample) {
        self.samples.push(sample);
    }
    pub fn set(&mut self, sample: Sample, frame: usize) {
        self.samples[frame] = sample;
    }
    pub fn push_wave(&mut self, wav: &Wave) {
        for idx in 0..wav.len() {
            let s = wav.get(idx);
            self.push(s);
        }
    }
}
