use super::wave::{
    Sample,
    Wave,
};

pub struct WaveTree {
    wav: Wave,
    branches: Vec<SampleBranch>,
}

#[derive(Clone, Copy)]
pub struct SampleBranch {
    data: Sample,
    frame: usize,
    left: usize,
    right: usize,
}

impl WaveTree {
}
