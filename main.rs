use macroquad::prelude::*;

// Define a 2D point
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

// Implement basic operations for Point
impl Point {
    fn lerp(self, other: Point, t: f32) -> Point {
        Point {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

// Apply one iteration of Chaikin's algorithm
fn chaikin(points: &[Point]) -> Vec<Point> {
    let mut new_points = Vec::new();
    if points.len() < 2 {
        return points.to_vec();
    }
    new_points.push(points[0]); // Keep the first point
    for i in 0..points.len() - 1 {
        let p0 = points[i];
        let p1 = points[i + 1];
        let q = p0.lerp(p1, 0.25);
        let r = p0.lerp(p1, 0.75);
        new_points.push(q);
        new_points.push(r);
    }
    new_points.push(points[points.len() - 1]); // Keep the last point
    new_points
}

#[macroquad::main("SilkenStroke")]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut smoothed_points: Vec<Point> = Vec::new();
    let mut animating = false;
    let mut step = 0;
    let max_steps = 7;
    let mut timer = 0.0;
    let delay = 0.5; // seconds between steps

    loop {
        clear_background(WHITE);

        // Handle mouse input
        if is_mouse_button_pressed(MouseButton::Left) && !animating {
            let mouse_pos = mouse_position();
            control_points.push(Point {
                x: mouse_pos.0,
                y: mouse_pos.1,
            });
        }

        // Handle keyboard input
        if is_key_pressed(KeyCode::Enter) && control_points.len() >= 2 {
            animating = true;
            step = 0;
            smoothed_points = control_points.clone();
            timer = 0.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Animation logic
        if animating {
            timer += get_frame_time();
            if timer >= delay {
                timer = 0.0;
                if step < max_steps {
                    smoothed_points = chaikin(&smoothed_points);
                    step += 1;
                } else {
                    // Restart animation
                    smoothed_points = control_points.clone();
                    step = 0;
                }
            }
        }

        // Draw control points
        for point in &control_points {
            draw_circle(point.x, point.y, 5.0, RED);
        }

        // Draw lines
        if control_points.len() == 1 {
            let p = control_points[0];
            draw_circle(p.x, p.y, 5.0, RED);
        } else if control_points.len() == 2 && !animating {
            let p0 = control_points[0];
            let p1 = control_points[1];
            draw_line(p0.x, p0.y, p1.x, p1.y, 2.0, BLACK);
        } else if animating {
            for i in 0..smoothed_points.len() - 1 {
                let p0 = smoothed_points[i];
                let p1 = smoothed_points[i + 1];
                draw_line(p0.x, p0.y, p1.x, p1.y, 2.0, BLUE);
            }
        }

        next_frame().await;
    }
}
