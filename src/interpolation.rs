pub trait Lerp {
    fn lerp(&self, rhs: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, rhs: Self, t: f32) -> Self {
        self * (1.0 - t) + rhs * t
    }
}

#[cfg(test)]
mod tests {
    use crate::interpolation::Lerp;
    #[test]
    fn lerp_f32() {
        let x: f32 = 0.0;
        assert_eq!(0.0, x.lerp(10.0, 0.0));
        assert_eq!(5.0, x.lerp(10.0, 0.5));
        assert_eq!(10.0, x.lerp(10.0, 1.0));
    }
}
