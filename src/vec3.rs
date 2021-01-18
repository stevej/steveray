use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {

    pub fn from(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x:x, y:y, z:z }
    }

    pub fn x(&self) -> &f64 { return &self.x }
    pub fn y(&self) -> &f64 { return &self.y }
    pub fn z(&self) -> &f64 { return &self.z }
    pub fn r(&self) -> &f64 { return &self.x }
    pub fn g(&self) -> &f64 { return &self.y }
    pub fn b(&self) -> &f64 { return &self.z }

    pub fn squared_length(&self) -> f64 {
        return self.x * self.x  + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64 {
        return self.squared_length().sqrt();
    }

    pub fn dot(&self, other: Self) -> f64 {
        return self.r() * other.r() + self.g() * other.g() + self.b() * other.b();
    }

    pub fn reflect(&self, other: Self) -> Self {
        let dot_product = self.dot(other);
        return *self - ((other * dot_product) * 2.0);
    }
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y  - self.y * other.x
        }
    }

    pub fn unit_vector(self) -> Self {
        let divisor = self.length();
        return Self {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
        }
    }
}

// a + b
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// a - b
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// a += b
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// a -= b
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// a * b
impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
// a *= b
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// a / b
impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// a / b
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

// a /= b
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// a /= b
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        let k = 1.0 / other;

        *self = Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;
    #[test]
    fn dot_product() {
        assert_eq!(Vec3{x:0.0,y:0.0,z:0.0}.dot(Vec3{x:0.0,y:0.0,z:0.0}), 0.0);
        assert_eq!(Vec3{x:1.0,y:2.0,z:3.0}.dot(Vec3{x:4.0,y:5.0,z:6.0}), 32.0);
    }

    #[test]
    fn cross_product() {
        assert_eq!(Vec3{x:0.0,y:0.0,z:0.0}.cross(Vec3{x:0.0,y:0.0,z:0.0}), Vec3{x:0.0,y:0.0,z:0.0});
        assert_eq!(Vec3{x:1.0,y:2.0,z:3.0}.cross(Vec3{x:4.0,y:5.0,z:6.0}), Vec3{x:-3.0,y:6.0,z:-3.0});
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3{x:1.0, y:2.0, z: 3.0}.length(), 3.7416573867739413);
    }

    #[test]
    fn add_vecs() {
        assert_eq!(Vec3{x:1.0, y:2.0, z:3.0} + Vec3{x: 1.0, y: 2.0, z: 3.0}, Vec3{x:2.0, y:4.0, z:6.0});
        let mut vec123 = Vec3{x:1.0, y:2.0, z:3.0};
        vec123 += Vec3{x: 1.0, y: 2.0, z: 3.0};
        assert_eq!(vec123 , Vec3{x:2.0, y:4.0, z:6.0})
    }
    #[test]
    fn subtract_vecs() {
        assert_eq!(Vec3{x:1.0, y:2.0, z:3.0} - Vec3{x:1.0, y:2.0, z:3.0}, Vec3{x:0.0, y:0.0, z:0.0});
        let mut vec123 = Vec3{x:1.0, y:2.0, z:3.0};
        let vec123_clone = vec123.clone();
        vec123 -= vec123_clone;
        assert_eq!(vec123, Vec3{x:0.0, y:0.0, z:0.0});
    }

    #[test]
    fn multiply_vecs() {
        assert_eq!(Vec3{x:1.0, y:2.0, z:3.0} * Vec3{x:2.0, y:2.0, z:2.0}, Vec3{x:2.0, y: 4.0, z: 6.0})
    }
}