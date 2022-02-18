struct Point {
    x: f64,
    y: f64,
}

struct Line {
    points: [Point; 2],
}

impl Line {
    fn list_points(self: &Self) {
        println!("A: ({}, {})\nB: ({}, {})", self.points[0].x, self.points[0].y, self.points[1].x, self.points[1].y);
    }
    fn distance(self: &Self) -> f64 {
        f64::sqrt(f64::powi(self.points[0].x - self.points[1].x, 2) + f64::powi(self.points[0].y - self.points[1].y, 2))
    }
}

fn main() {
    let my_line = Line {
        points: [
            Point{x: 0.0, y: 1.0},
            Point{x: 3.0,y: 4.0}
        ],
    };

    my_line.list_points();
    println!("Distance: {}", my_line.distance());
}
