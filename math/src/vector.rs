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

macro_rules! zip_arithmetic {
    ($Scalar:ty, $op:ident) => {
        #[inline]
        fn $op(self, other: Self) -> Self {
            self.zip_map(other, <$Scalar>::$op)
        }
    };
}

macro_rules! zip_arithmetic_assign {
    ($Scalar:ty, $op:ident, $op_assign:ident) => {
        #[inline]
        fn $op_assign(&mut self, other: Self) {
            self.zip_map_assign(other, <$Scalar>::$op)
        }
    };
}

macro_rules! impl_binary_op {
    ($Vector:ty, $Scalar:ty, $Trait:ident, $TraitAssign:ident, $op:ident, $op_assign:ident) => {
        impl $Trait for $Vector {
            type Output = Self;
            zip_arithmetic!($Scalar, $op);
        }

        impl $TraitAssign for $Vector {
            zip_arithmetic_assign!($Scalar, $op, $op_assign);
        }
    };
}

impl_binary_op!(Vec3f, f32, Add, AddAssign, add, add_assign);
impl_binary_op!(Vec3f, f32, Sub, SubAssign, sub, sub_assign);

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