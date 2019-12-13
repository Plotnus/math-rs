pub mod vector;
pub mod point;
pub mod lerp;

// We use `Rad` for our internal structure since this is the default for trig ops
#[derive(Copy,Clone, PartialEq, Debug)]
struct Rad(f32);
impl From<Deg> for Rad {
    fn from(d: Deg) -> Rad {
        use std::f32::consts::PI;
        const DEG_TO_RAD: f32 = PI / 180.0;
        Rad(DEG_TO_RAD * d.0)
    }
}
fn sin(r: Rad) -> f32 {
    r.0.sin()
}
fn cos(r: Rad) -> f32 {
    r.0.cos()
}




// Degrees should only be used for interfacing with end users
#[derive(Copy,Clone, PartialEq, Debug)]
struct Deg(f32);
impl From<Rad> for Deg {
    fn from(r: Rad) -> Deg {
        use std::f32::consts::PI;
        const RAD_TO_DEG: f32 = 180.0 / PI;
        Deg(RAD_TO_DEG * r.0)
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
}
