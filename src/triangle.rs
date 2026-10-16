use crate::framebuffer::Framebuffer;
use glam::Vec3;

fn edge(a: Vec3, b: Vec3, x: f32, y: f32) -> f32 {
    (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
}

/// Rellena un triangulo con coordenadas baricentricas, pixel por pixel en CPU.
pub fn triangle(buffer: &mut Framebuffer, a: Vec3, b: Vec3, c: Vec3) {
    let area = edge(a, b, c.x, c.y);
    if area.abs() < 1e-6 {
        return;
    }
    let min_x = a.x.min(b.x).min(c.x).floor().max(0.0) as i32;
    let max_x = a.x.max(b.x).max(c.x).ceil().min(buffer.width as f32 - 1.0) as i32;
    let min_y = a.y.min(b.y).min(c.y).floor().max(0.0) as i32;
    let max_y = a.y.max(b.y).max(c.y).ceil().min(buffer.height as f32 - 1.0) as i32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // Evaluar el centro del pixel.
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let wa = edge(b, c, px, py) / area;
            let wb = edge(c, a, px, py) / area;
            let wc = edge(a, b, px, py) / area;
            if wa >= -1e-6 && wb >= -1e-6 && wc >= -1e-6 {
                let depth = wa * a.z + wb * b.z + wc * c.z;
                buffer.point(x, y, depth);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{Color, BACKGROUND};

    #[test]
    fn depth_is_independent_of_drawing_order() {
        let mut first = Framebuffer::new(100, 100);
        let mut second = Framebuffer::new(100, 100);
        for (buffer, order) in [(&mut first, [0.8, 0.2]), (&mut second, [0.2, 0.8])] {
            for z in order {
                buffer.set_color(if z < 0.5 {
                    Color::new(0, 255, 0)
                } else {
                    Color::new(255, 0, 0)
                });
                triangle(
                    buffer,
                    Vec3::new(10.0, 10.0, z),
                    Vec3::new(80.0, 10.0, z),
                    Vec3::new(10.0, 80.0, z),
                );
            }
        }
        assert_eq!(first.pixels, second.pixels);
        let offset = (20 * 100 + 20) * 4;
        assert_eq!(&first.pixels[offset..offset + 4], &[0, 255, 0, 255]);
    }

    #[test]
    fn both_vertex_orders_produce_same_pixels() {
        let (a, b, c) = (
            Vec3::new(10.0, 10.0, 0.5),
            Vec3::new(80.0, 10.0, 0.5),
            Vec3::new(10.0, 80.0, 0.5),
        );
        let mut buffer = Framebuffer::new(100, 100);
        triangle(&mut buffer, a, b, c);
        let reference = buffer.pixels.clone();
        buffer.clear();
        triangle(&mut buffer, a, c, b);
        assert_eq!(reference, buffer.pixels);
    }

    #[test]
    fn degenerate_triangle_does_not_paint() {
        let mut buffer = Framebuffer::new(100, 100);
        let reference = buffer.pixels.clone();
        triangle(
            &mut buffer,
            Vec3::splat(10.0),
            Vec3::splat(10.0),
            Vec3::splat(10.0),
        );
        assert_eq!(reference, buffer.pixels);
    }

    #[test]
    fn triangle_crossing_screen_edge_is_bounded() {
        let mut buffer = Framebuffer::new(100, 100);
        triangle(
            &mut buffer,
            Vec3::new(-50.0, -50.0, 0.5),
            Vec3::new(50.0, -50.0, 0.5),
            Vec3::new(-50.0, 150.0, 0.5),
        );
        assert_eq!(&buffer.pixels[0..4], &[255, 255, 255, 255]);
        assert_eq!(
            &buffer.pixels[99 * 4..100 * 4],
            &[BACKGROUND.r, BACKGROUND.g, BACKGROUND.b, 255]
        );
    }
}
