use crate::vec3::Vec3;

pub struct Ray {
    A: Vec3,
    B: Vec3,
}

impl Ray {
    pub fn from(a: Vec3, b: Vec3) -> Self {
        Ray { A: a, B: b}
    }

    pub fn origin(&self) -> Vec3 {
        return self.A
    }

    pub fn direction(&self) -> Vec3 {
        return self.B
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        return self.A + self.B*t
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        assert_eq!(true, true);
    }
}