use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) -> String {
    // translate the pixel's color value from the range [0, 1] -> [0, 255]
    let rbyte = (255.999 * pixel_color.x()) as i32;
    let gbyte = (255.999 * pixel_color.y()) as i32;
    let bbyte = (255.999 * pixel_color.z()) as i32;
    format!("{} {} {}\n", rbyte, gbyte, bbyte)
}
