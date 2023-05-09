use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3f(pub [f32; 3]);

impl Vec3f {
    #[inline]
    pub fn zip_map(self, other: Self, f: impl Fn(f32, f32) -> f32) -> Self {
        let (lhs, rhs) = (self.0, other.0);
        Self([f(lhs[0], rhs[0]), f(lhs[1], rhs[1]), f(lhs[2], rhs[2])])
    }

    #[inline]
    pub fn zip_map_assign(
        &mut self,
        other: Self,
        f: impl Fn(f32, f32) -> f32,
    ) {
        *self = self.zip_map(other, f);
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        let (lhs, rhs) = (self.0, other.0);
        Self([
            lhs[1] * rhs[2] - lhs[2] * rhs[1],
            lhs[2] * rhs[0] - lhs[0] * rhs[2],
            lhs[0] * rhs[1] - lhs[1] * rhs[0],
        ])
    }
}

impl Add for Vec3f {
    type Output = Self;

    #[inline]
    fn add(&self, other: Self) -> Self {
        self.zip_map(f32::add)
    }
}

impl AddAssign for Vec3f {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.zip_map_assign(f32::add)
    }
}

impl Sub for Vec3f {
    type Output = Self;

    #[inline]
    fn sub(&self, other: Self) -> Self {
        self.zip_map(f32::sub)
    }
}

impl SubAssign for Vec3f {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.zip_map_assign(f32::sub)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3f;

    const BASIS_I: Vec3f = Vec3f([1.0f32, 0.0f32, 0.0f32]);
    const BASIS_J: Vec3f = Vec3f([0.0f32, 1.0f32, 0.0f32]);
    const BASIS_K: Vec3f = Vec3f([0.0f32, 0.0f32, 1.0f32]);

    #[test]
    fn basis_cross_products_each_other() {
        assert_eq!(BASIS_K, BASIS_I.cross(BASIS_J));
        assert_eq!(BASIS_I, BASIS_J.cross(BASIS_K));
        assert_eq!(BASIS_J, BASIS_K.cross(BASIS_I));
    }

    #[test]
    fn add_sub_basics() {
        let a = Vec3f([1.0f32, 2.0f32, 3.0f32]);
        let b = Vec3f([4.0f32, 5.0f32, 6.0f32]);
        assert_eq!(a + b, Vec3f([5.0f32, 7.0f32, 9.0f32]));
        let mut c = b;
        c -= a;
        assert_eq!(c, Vec3f([3.0f32, 3.0f32, 3.0f32]));
        assert_eq!(b - c, a);
        c += a;
        assert_eq!(c, b);
    }
}