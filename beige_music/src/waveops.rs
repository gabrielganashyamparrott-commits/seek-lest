use super::wave::{
    Sample,
    Wave,
    Channels,
};

use std::ops::{
    Add,
    Sub,
    Div,
    Mul,
    Rem,
    Neg,
};

impl Add for Sample {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        add(self, other)
    }
}

impl Add<f32> for Sample {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        offset(self, other)
    }
}

impl Add<Sample> for f32 {
    type Output = Sample;

    fn add(self, other: Sample) -> Sample {
        offset(other, self)
    }
}

impl Sub for Sample {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        sub(self, other)
    }
}

impl Sub<f32> for Sample {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        offset(self, -1f32 * other)
    }
}

impl Sub<Sample> for f32 {
    type Output = Sample;

    fn sub(self, other: Sample) -> Sample {
        -(other - self)
    }
}

impl Div for Sample {
    type Output = Self;

    fn div(self, other: Sample) -> Self {
        div(self, other)
    }
}

impl Div<f32> for Sample {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        scale(self, 1f32 / other)
    }
}

impl Mul for Sample {
    type Output = Self;

    fn mul(self, other: Sample) -> Self {
        mul(self, other)
    }
}

impl Mul<f32> for Sample {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        scale(self, other)
    }
}

impl Rem<f32> for Sample {
    type Output = Self;

    fn rem(self, other: f32) -> Self {
        rem(self, other)
    }
}

impl Neg for Sample {
    type Output = Self;
    fn neg(self) -> Self {
        neg(self)
    }
}

pub fn rotate_cc(x: f32, y: f32, angle: f32) -> Sample {
    let u = (x * angle.cos()) - (y * angle.sin());
    let v = (x * angle.sin()) + (y * angle.cos());

    Sample::Stereo {left: u, right: v}
}

pub fn rotate_cw(x: f32, y: f32, angle: f32) -> Sample {
    let u = (x * angle.cos()) + (y * angle.sin());
    let v = (x * angle.sin() * -1.0) + (y * angle.cos());

    Sample::Stereo {left: u, right: v}
}

pub fn channels(s: Sample) -> Channels {
    match s {
        Sample::Mono(_) => Channels::One,
        Sample::Stereo {left: _a, right: _b} => Channels::Two,
    }
}

pub fn channels_w(w: &Wave) -> Channels {
    if w.channels() < 2 {
        Channels::One
    } else {
        Channels::Two
    }
}

pub fn zero(ch: Channels) -> Sample {
    match ch {
        Channels::One => Sample::Mono(0.0),
        Channels::Two => Sample::Stereo {left: 0.0, right: 0.0},
    }
}

pub fn add(lhs: Sample, rhs: Sample) -> Sample {
    match lhs {
        Sample::Mono(x) => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Mono(x + a)
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x + a, right: x + b}
                },
            }
        },
        Sample::Stereo {left: x, right: y} => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Stereo {left: x + a, right: y + a}
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x + a, right: y + b}
                },
            }
        },
    }
}

pub fn rem(lhs: Sample, rhs: f32) -> Sample {
    match lhs {
        Sample::Mono(x) => {
            Sample::Mono(x % rhs)
        },
        Sample::Stereo {left: x, right: y} => {
            Sample::Stereo {
                left: x % rhs,
                right: y % rhs,
            }
        },
    }
}

pub fn sub(lhs: Sample, rhs: Sample) -> Sample {
    match lhs {
        Sample::Mono(x) => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Mono(x - a)
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x - a, right: x - b}
                },
            }
        },
        Sample::Stereo {left: x, right: y} => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Stereo {left: x - a, right: y - a}
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x - a, right: y - b}
                },
            }
        },
    }
}

pub fn mul(lhs: Sample, rhs: Sample) -> Sample {
    match lhs {
        Sample::Mono(x) => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Mono(x * a)
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x * a, right: x * b}
                },
            }
        },
        Sample::Stereo {left: x, right: y} => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Stereo {left: x * a, right: y * a}
                },
                Sample::Stereo {left: a, right: b} => {
                    let re = (x * a) + (y * b);
                    let im = (x * b) + (y * a);
                    Sample::Stereo {left: re, right: im}
                },
            }
        },
    }
}

pub fn div(lhs: Sample, rhs: Sample) -> Sample {
    match lhs {
        Sample::Mono(x) => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Mono(x / a)
                },
                Sample::Stereo {left: a, right: b} => {
                    let ln = (x * a) + (x * b);
                    let rn = (x * a) - (x * b);
                    let d = a.powf(2f32) + b.powf(2f32);
                    Sample::Stereo {left: ln / d, right: rn / d}
                },
            }
        },
        Sample::Stereo {left: x, right: y} => {
            match rhs {
                Sample::Mono(a) => {
                    let ln = (x*a) + (y*a);
                    let rn = (y*a) - (x*a);
                    let d = 2f32 * a.powf(2f32);
                    Sample::Stereo {left: ln / d, right: rn / d}
                },
                Sample::Stereo {left: a, right: b} => {
                    let ln = (x * a) + (y * b);
                    let rn = (y * a) - (x * b);
                    let d = a.powf(2f32) + b.powf(2f32);
                    Sample::Stereo {left: ln / d, right: rn / d}
                },
            }
        },
    }
}

pub fn avg(lhs: Sample, rhs: Sample) -> Sample {
    match lhs {
        Sample::Mono(x) => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Mono((x + a) / 2.0)
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: (x + a) / 2.0, right: (x + b) / 2.0}
                },
            }
        },
        Sample::Stereo {left: x, right: y} => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Stereo {left: (x + a) / 2.0, right: (y + a) / 2.0}
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: (x + a) / 2.0, right: (y + b) / 2.0}
                },
            }
        },
    }
}

pub fn neg(s: Sample) -> Sample {
    match s {
        Sample::Mono(x) => {
            Sample::Mono(x * -1.0)
        },
        Sample::Stereo {left: x, right: y} => {
            Sample::Stereo {left: x * -1.0, right: y * -1.0}
        },
    }
}

pub fn mag(s: Sample) -> f32 {
    match s {
        Sample::Mono(x) => {
            x.abs()
        },
        Sample::Stereo {left: x, right: y} => {
            (x.powf(2f32) + y.powf(2f32)).sqrt()
        },
    }
}

pub fn scale(s: Sample, f: f32) -> Sample {
    match s {
        Sample::Mono(x) => {
            Sample::Mono(x * f)
        },
        Sample::Stereo {left: x, right: y} => {
            Sample::Stereo {left: x * f, right: y * f}
        }
    }
}

pub fn offset(s: Sample, o: f32) -> Sample {
    match s {
        Sample::Mono(x) => {
            Sample::Mono(x + o)
        },
        Sample::Stereo {left: x, right: y} => {
            Sample::Stereo {left: x + o, right: y + o}
        }
    }
}

pub fn powf(s: Sample, p: f32) -> Sample {
    match s {
        Sample::Mono(x) => {
            Sample::Mono(x.powf(p))
        },
        Sample::Stereo { left: x, right: y } => {
            Sample::Stereo { left: x.powf(p), right: y.powf(p) }
        }
    }
}

pub fn concat(lhs: &Wave, rhs: &Wave) -> Wave {
    let mut out = Wave::new(lhs.samplerate(), lhs.channels());
    // push both Waves
    out.push_wave(lhs);
    let blend_at = out.len() - 1;
    out.push_wave(rhs);

    // mend the seam
    blendo(&mut out, blend_at);

    // return the concatenated Wave
    out
}

pub fn blendo(wav: &mut Wave, at: usize) {
    let at = at.clamp(0, wav.len() - 1);

    let ll = (at - 2).clamp(0, at);
    let l = (at - 1).clamp(0, at);
    let r = (at + 1).clamp(0, wav.len() - 1);
    let rr = (at + 2).clamp(0, wav.len() - 1);
    let lls = wav.get(ll);
    let ls = wav.get(l);
    let rs = wav.get(r);
    let rrs = wav.get(rr);
    let blendout = avg(lls, rrs);
    let blendl = avg(blendout, avg(ls, lls));
    let blendm = avg(blendout, avg(ls, rs));
    let blendr = avg(blendout, avg(rs, rrs));
    wav.set(blendl, ll);
    wav.set(blendm, l);
    wav.set(blendm, r);
    wav.set(blendr, rr);
}

pub fn blendos(wav: &Wave, at: usize) -> Sample {
    let at = at.clamp(0, wav.len() - 1);

    let ll = (at - 2).clamp(0, at);
    let l = (at - 1).clamp(0, at);
    let r = (at + 1).clamp(0, wav.len() - 1);
    let rr = (at + 2).clamp(0, wav.len() - 1);
    let lls = wav.get(ll);
    let ls = wav.get(l);
    let rs = wav.get(r);
    let rrs = wav.get(rr);
    let blendout = avg(lls, rrs);
    let blendl = avg(blendout, avg(ls, lls));
    let blendm = avg(blendout, avg(ls, rs));
    let blendr = avg(blendout, avg(rs, rrs));

    avg(blendm, avg(blendl, blendr))
}

pub fn stretch(
    wav: &Wave,
    start_frame: usize,
    end_frame: usize,
    factor: f32,
) -> Wave {
    // render stretch to a new Wave
    let samplerate = wav.samplerate();
    let channels = wav.channels();
    let mut out = Wave::new(samplerate, channels);

    // number of frames in result
    let new_frames = ((end_frame - start_frame) as f32 * factor) as usize;

    // iterate through the source file at a rate of 1.0 / factor per iteration
    let mut src_iter = start_frame as f32;
    let mut step_past = 0;
    let src_step = 1.0 / factor;

    // slope stretch
    while out.len() < new_frames {
        let src_frame = src_iter.floor() + step_past as f32;
        let sample = blendos(&wav, (src_frame as usize).clamp(0, wav.len() - 1));

        out.push(sample);

        src_iter += src_step;
        step_past = (step_past + 1) % (factor * 2.0) as usize;
    };

    out
}

// TODO pluck

pub fn beats_to_samples(
    beats: f32,
    bpm: f32,
    samplerate: usize
) -> usize {
    let seconds = beats_to_seconds(beats, bpm);
    (seconds * samplerate as f32) as usize
}

pub fn beats_to_seconds(
    beats: f32,
    bpm: f32
) -> f32 {
    let bps = bpm / 60f32;
    
    beats / bps
}
