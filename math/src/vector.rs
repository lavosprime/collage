#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3f(pub [f32; 3]);

impl Vec3f {
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
}