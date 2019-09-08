#[inline]
pub fn get_point_on_radius(rad: f32, angle: f32) -> [f32; 2] {
    [rad * angle.cos(), rad * angle.sin()]
}
