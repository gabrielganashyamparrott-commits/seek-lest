use super::wave::{
    Sample,
    Wave,
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
