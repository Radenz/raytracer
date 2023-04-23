use crate::vec::Vector3;

impl Into<[u8; 3]> for Vector3 {
    fn into(self) -> [u8; 3] {
        [self.x() as u8, self.y() as u8, self.z() as u8]
    }
}

pub trait Vector3Extension {
    fn to_u8_range(&self) -> Vector3;
}

impl Vector3Extension for Vector3 {
    fn to_u8_range(&self) -> Vector3 {
        let mut u8s = self.clamp_each(0, 1);
        u8s *= 256;
        u8s
    }
}
