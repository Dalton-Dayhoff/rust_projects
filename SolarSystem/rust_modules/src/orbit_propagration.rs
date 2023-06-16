
const GRAVITATIONAL_CONSTANT: f64 = 6.6743E-11;
const MASS_OF_SUN: f64 = 1.99E30;

pub fn get_mu(body1: Body, body2: Body) -> f64{
    GRAVITATIONAL_CONSTANT*(body1::mass as f64 + body2::mass as f64)
}

