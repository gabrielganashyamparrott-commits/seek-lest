use super::wave::{
    Sample,
    Wave,
    Channels,
};

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

pub fn rem(lhs: Sample, rhs: Sample) -> Sample {
    match lhs {
        Sample::Mono(x) => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Mono(x % a)
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x % a, right: x % b}
                },
            }
        },
        Sample::Stereo {left: x, right: y} => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Stereo {left: x % a, right: y % a}
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x % a, right: y % b}
                },
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
                    Sample::Stereo {left: x * a, right: y * b}
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
                    Sample::Stereo {left: x / a, right: x / b}
                },
            }
        },
        Sample::Stereo {left: x, right: y} => {
            match rhs {
                Sample::Mono(a) => {
                    Sample::Stereo {left: x / a, right: y / a}
                },
                Sample::Stereo {left: a, right: b} => {
                    Sample::Stereo {left: x / a, right: y / b}
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
    let src_step = 1.0 / factor;

    let mut accum = wav.get(start_frame);
    while out.len() < new_frames {
        let src_frame = src_iter.floor() as usize;
        accum = avg(accum, wav.get(src_frame));
        src_iter += src_step;
    };

    out
}
