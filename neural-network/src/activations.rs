use std::f64::consts::E;



#[derive(Clone,Copy,Debug)]
pub struct Activation {
    pub function: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64,
}

pub const SIGMOID: Activation = Activation {
    function: |x| 1.0 / (1.0 + E.powf(-x)),
    derivative: |x| x * (1.0 - x),
};
