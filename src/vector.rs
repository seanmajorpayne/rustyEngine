use std::cmp;

struct Vec2D {
    x: f64,
    y: f64,
}

struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

enum Vector {
    V2(Vec2D),
    V3(Vec3D),
}

impl Vec2D {
    fn draw(size: u16) {
        // TODO
    }

    fn magnitude(&self) -> f64 {
        return (self.x*self.x + self.y*self.y).sqrt();
    }

    fn magnitude_squared(&self) -> f64 {
        return self.x*self.x + self.y*self.y
    }

    fn add(&mut self, other: Vec2D) {
        self.x += other.x;
        self.y += other.y;
    }

    fn sub(&mut self, other: Vec2D) {
        self.x -= other.x;
        self.y -= other.y;
    }

    fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    fn dot(&self, other: Vec2D) -> f64 {
        return self.x * other.x + self.y * other.y;
    }

    fn cross(&self, other: Vec2D) -> Vec3D {
        return Vec3D { x: 0.0, y: 0.0, z: self.x * other.y - self.y * other.x }
    }

    fn perpendicular(&self) -> Vec2D {
        return Vec2D { x: self.y, y: -self.x}
    }

    fn normalize(&mut self) {
        let mag: f64 = self.magnitude();
        if mag != 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
    }

    fn unit_vector(&self) -> Vec2D {
        let mut v = Vec2D { x: 0.0, y: 0.0 };
        let mag: f64 = self.magnitude();
        if mag != 0.0 {
            v.x = self.x / mag;
            v.y = self.y / mag;
        }
        return v
    }

    fn transform(&mut self) {
        // TODO
    }

    fn rotate(&mut self, theta: f64) {
        self.x = self.x * theta.cos() - self.y * theta.sin();
        self.y = self.x * theta.sin() + self.y * theta.cos();
    }
}

impl Vec3D {
    fn draw(size: u16) {
        // TODO
    }

    fn magnitude(&self) -> f64 {
        return (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    fn add(&mut self, other: Vec3D) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }

    fn sub(&mut self, other: Vec3D) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }

    fn scale(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    fn dot(&self, other: Vec3D) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    fn cross(&mut self, other: Vec3D) {
        self.x = self.y * other.z - self.z * other.y;
        self.y = self.z * other.x - self.x * other.z;
        self.z = self.x * other.y - self.y * other.x;
    }
}



