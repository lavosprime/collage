use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Neg;
use core::ops::Sub;
use core::ops::SubAssign;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3f {
    components: [f32; 3],
}

impl Vec3f {
    pub const BASIS_0: Self = Self::new(1.0, 0.0, 0.0);
    pub const BASIS_1: Self = Self::new(0.0, 1.0, 0.0);
    pub const BASIS_2: Self = Self::new(0.0, 0.0, 1.0);

    // Argument names abbreviated from `component_0` etc due to noise in IDE parameter name hints.
    #[inline]
    pub const fn new(c0: f32, c1: f32, c2: f32) -> Self {
        Self { components: [c0, c1, c2] }
    }

    #[inline]
    pub fn map(self, f: impl Fn(f32) -> f32) -> Self {
        Self { components: self.components.map(f) }
    }

    #[inline]
    pub fn zip_map(self, other: Self, f: impl Fn(f32, f32) -> f32) -> Self {
        let (lhs, rhs) = (self.components, other.components);
        Self {
            components: [
                f(lhs[0], rhs[0]),
                f(lhs[1], rhs[1]),
                f(lhs[2], rhs[2]),
            ],
        }
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
        let (lhs, rhs) = (self.components, other.components);
        Self {
            components: [
                lhs[1] * rhs[2] - lhs[2] * rhs[1],
                lhs[2] * rhs[0] - lhs[0] * rhs[2],
                lhs[0] * rhs[1] - lhs[1] * rhs[0],
            ],
        }
    }
}

impl From<[f32; 3]> for Vec3f {
    #[inline]
    fn from(components: [f32; 3]) -> Self {
        Vec3f { components }
    }
}

impl Into<[f32; 3]> for Vec3f {
    #[inline]
    fn into(self) -> [f32; 3] {
        self.components
    }
}

impl Index<usize> for Vec3f {
    type Output = f32;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        self.components.index(i)
    }
}

impl IndexMut<usize> for Vec3f {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        self.components.index_mut(i)
    }
}

macro_rules! map_arithmetic {
    ($Scalar:ty, $op:ident) => {
        #[inline]
        fn $op(self) -> Self {
            self.map(<$Scalar>::$op)
        }
    };
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

macro_rules! impl_unary_op {
    ($Vector:ty, $Scalar:ty, $Trait:ident, $op:ident) => {
        impl $Trait for $Vector {
            type Output = Self;
            map_arithmetic!($Scalar, $op);
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

impl_unary_op!(Vec3f, f32, Neg, neg);
impl_binary_op!(Vec3f, f32, Add, AddAssign, add, add_assign);
impl_binary_op!(Vec3f, f32, Sub, SubAssign, sub, sub_assign);

#[cfg(test)]
mod tests {
    use super::Vec3f;

    const BASIS_0: Vec3f = Vec3f::BASIS_0;
    const BASIS_1: Vec3f = Vec3f::BASIS_1;
    const BASIS_2: Vec3f = Vec3f::BASIS_2;

    #[test]
    fn basis_cross_products_each_other() {
        assert_eq!(BASIS_2, BASIS_0.cross(BASIS_1));
        assert_eq!(BASIS_0, BASIS_1.cross(BASIS_2));
        assert_eq!(BASIS_1, BASIS_2.cross(BASIS_0));
        assert_eq!(-BASIS_2, BASIS_1.cross(BASIS_0));
        assert_eq!(-BASIS_0, BASIS_2.cross(BASIS_1));
        assert_eq!(-BASIS_1, BASIS_0.cross(BASIS_2));
    }

    #[test]
    fn add_sub_basics() {
        let a = Vec3f::new(1.0f32, 2.0f32, 3.0f32);
        let b = Vec3f::new(4.0f32, 5.0f32, 6.0f32);
        assert_eq!(a + b, Vec3f::new(5.0f32, 7.0f32, 9.0f32));
        let mut c = b;
        c -= a;
        assert_eq!(c, Vec3f::new(3.0f32, 3.0f32, 3.0f32));
        assert_eq!(b - c, a);
        c += a;
        assert_eq!(c, b);
    }

    //
    // Here be ChatGPT lol
    //

    #[test]
    fn test_from_into() {
        let array: [f32; 3] = [1.0, 2.0, 3.0];

        let vector: Vec3f = Vec3f::from(array);
        let converted_array: [f32; 3] = vector.into();

        assert_eq!(array, converted_array);
    }

    #[test]
    fn test_index() {
        let vector = Vec3f::new(1.0, 2.0, 3.0);

        assert_eq!(vector[0], 1.0);
        assert_eq!(vector[1], 2.0);
        assert_eq!(vector[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut vector = Vec3f::new(1.0, 2.0, 3.0);

        vector[0] = 4.0;
        vector[1] = 5.0;
        vector[2] = 6.0;

        assert_eq!(vector[0], 4.0);
        assert_eq!(vector[1], 5.0);
        assert_eq!(vector[2], 6.0);
    }

    #[test]
    fn test_unary_negation() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);

        let result = -v1;
        let expected_result = Vec3f::new(-1.0, -2.0, -3.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_componentwise_addition() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);

        let result = v1 + v2;
        let expected_result = Vec3f::new(5.0, 7.0, 9.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_componentwise_subtraction() {
        let v1 = Vec3f::new(4.0, 5.0, 6.0);
        let v2 = Vec3f::new(1.0, 2.0, 3.0);

        let result = v1 - v2;
        let expected_result = Vec3f::new(3.0, 3.0, 3.0);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3f::new(1.0, 2.0, 3.0);
        let v2 = Vec3f::new(4.0, 5.0, 6.0);

        let cross_product1 = v1.cross(v2);
        let expected_cross_product1 = Vec3f::new(-3.0, 6.0, -3.0);

        assert_eq!(cross_product1, expected_cross_product1);

        let v3 = Vec3f::new(-2.0, 3.0, -1.0);
        let v4 = Vec3f::new(4.0, -1.0, 5.0);

        let cross_product2 = v3.cross(v4);
        let expected_cross_product2 = Vec3f::new(14.0, 6.0, -10.0);

        assert_eq!(cross_product2, expected_cross_product2);
    }
}
