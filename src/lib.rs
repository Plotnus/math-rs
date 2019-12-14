#![feature(clamp)]
pub mod vec3;
pub mod vec4;
pub mod point;

// We use `Rad` for our internal structure since this is the default for trig ops
#[derive(Copy,Clone, PartialEq, Debug)]
pub struct Rad(f32);
impl From<Deg> for Rad {
    fn from(d: Deg) -> Rad {
        Rad(d.0.to_radians())
    }
}

pub fn sin(r: Rad) -> f32 {
    r.0.sin()
}

pub fn cos(r: Rad) -> f32 {
    r.0.cos()
}




// Degrees should only be used for interfacing with end users
#[derive(Copy,Clone, PartialEq, Debug)]
pub struct Deg(f32);
impl From<Rad> for Deg {
    fn from(r: Rad) -> Deg {
        Deg(r.0.to_degrees())
    }
}



#[cfg(test)]
mod tests {
    use crate::Deg;
    use crate::Rad;
    #[test]
    fn rad_to_deg() {
        assert_eq!(Deg(-360.0), Rad(-2.0 * std::f32::consts::PI).into());
        assert_eq!(Deg(-180.0), Rad(-std::f32::consts::PI).into());
        assert_eq!(Deg(0.0), Rad(0.0).into());
        assert_eq!(Deg(180.0), Rad(std::f32::consts::PI).into());
        assert_eq!(Deg(360.0), Rad(2.0 * std::f32::consts::PI).into());
    }
    #[test]
    fn deg_to_rad() {
        assert_eq!(Rad(-2.0 * std::f32::consts::PI), Deg(-360.0).into());
        assert_eq!(Rad(-std::f32::consts::PI), Deg(-180.0).into());
        assert_eq!(Rad(0.0), Deg(0.0).into());
        assert_eq!(Rad(std::f32::consts::PI), Deg(180.0).into());
        assert_eq!(Rad(2.0 * std::f32::consts::PI), Deg(360.0).into());
    }

    #[test]
    fn cos() {
        use std::f32::consts::{PI, FRAC_PI_2};
        let angles = [ 0.0, FRAC_PI_2, PI, -FRAC_PI_2];
        for angle in angles.iter() {
            assert_eq!(angle.cos(), crate::cos(Rad(*angle)));
        }
    }

    #[test]
    fn sin() {
        use std::f32::consts::{PI, FRAC_PI_2};
        let angles = [ 0.0, FRAC_PI_2, PI, -FRAC_PI_2];
        for angle in angles.iter() {
            assert_eq!(angle.sin(), crate::sin(Rad(*angle)));
        }
    }
}
