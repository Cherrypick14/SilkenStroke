// src/lib.rs

#[derive(Clone)] // Derive Clone to allow cloning of Point
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    // Linear interpolation (lerp) between two points
    pub fn lerp(&self, other: &Point, t: f32) -> Point {
        Point {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

// Chaikin's algorithm function
pub fn chaikin(points: &[Point]) -> Vec<Point> {
    let mut new_points = Vec::new();
    if points.len() < 2 {
        return points.to_vec(); // No change if there are less than 2 points
    }
    // Keep the first point
    new_points.push(points[0].clone());
    
    // Iterate through the points, adding new points according to Chaikin's algorithm
    for i in 0..points.len() - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];
        let q = p0.lerp(p1, 0.25);
        let r = p0.lerp(p1, 0.75);
        new_points.push(q);
        new_points.push(r);
    }

    // Keep the last point
    new_points.push(points[points.len() - 1].clone());
    new_points
}
