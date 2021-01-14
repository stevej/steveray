use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn from(a: Vec3, b: Vec3) -> Self {
        Ray { a: a, b: b}
    }

    pub fn origin(&self) -> Vec3 {
        return self.a
    }

    pub fn direction(&self) -> Vec3 {
        return self.b
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        return self.a + self.b*t
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        assert_eq!(true, true);
    }
}