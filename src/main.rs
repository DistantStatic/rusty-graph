#[derive(Copy, Clone)]
/// Building block for all shapes.
/// Takes an X and Y coordinate to make a Point
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }
}

/// Line from two Points
struct Line {
    points: [Point; 2],
}

impl Line {
    fn new(p0: Point, p1: Point) -> Line {
        Line {
            points: [
                p0,
                p1,
            ]
        }
    }
    fn list_points(self: &Self) {
        println!("A: ({}, {})\nB: ({}, {})", self.points[0].x, self.points[0].y, self.points[1].x, self.points[1].y);
    }
    fn distance(self: &Self) -> f64 {
        f64::sqrt(f64::powi(self.points[0].x - self.points[1].x, 2) + f64::powi(self.points[0].y - self.points[1].y, 2))
    }
    fn list_distance(self: &Self) {
        println!("Distance: {}", self.distance());
    }
    fn flip(self: &mut Self) {
        let old_point = self.points[0];
        self.points[0] = self.points[1];
        self.points[1] = old_point;
    }
    fn translate_x(self: &mut Self, amount: f64) {
        self.points[0].x = self.points[0].x + amount;
        self.points[1].x = self.points[1].x + amount;
    }
    fn translate_y(self: &mut Self, amount: f64) {
        self.points[0].y = self.points[0].y + amount;
        self.points[1].y = self.points[1].y + amount;
    }
    fn reflect_x(self: &mut Self) {
        self.points[0].y = self.points[0].y * -1.0;
        self.points[1].y = self.points[1].y * -1.0;
    }
    fn reflect_y(self: &mut Self) {
        self.points[0].x = self.points[0].x * -1.0;
        self.points[1].x = self.points[1].x * -1.0;
    }
    fn find_slope(self: &Self) -> f64{
        (self.points[1].y - self.points[0].y) / (self.points[1].x - self.points[0].x)
    }
    fn list_slope(self: &Self) {
        println!("Slope: {}", self.find_slope());
    }
    fn details(self: &Self) {
        self.list_points();
        self.list_distance();
        self.list_slope();
    }
}

struct Triangle{
    points: [Point; 3],
}

impl Triangle {
    fn new(p0: Point, p1: Point, p2: Point) -> Triangle {
        Triangle {
            points: [
                p0,
                p1,
                p2,
            ]
        }
    }
    fn reflect_x(self: &mut Self) {
        for mut p in self.points {
            p.y = p.y * -1.0;
        }
    }
    fn reflect_y(self: &mut Self) {
        for mut p in self.points {
            p.x = p.x * -1.0;
        }
    }
}

struct Quadrilateral{
    points: [Point; 4],
}

impl Quadrilateral{
    fn new(p0: Point, p1: Point, p2: Point, p3: Point) -> Quadrilateral{
        Quadrilateral{
            points: [ p0, p1, p2, p3 ],
        }
    }
    fn get_side_line(self: &Self, side: usize) -> Line {
        let start = side % 4;
        let end = (start + 1) % 4;

        Line::new(
            Point{
                x: self.points[start].x,
                y: self.points[start].y,
            },
            Point{
                x: self.points[end].x,
                y: self.points[end].y,
            }
        )
    }

}
        
fn main() {
    let mut my_line = Line::new(
            Point{x: 0.0, y: 1.0},
            Point{x: 3.0,y: 4.0}
        );

    let my_square = Quadrilateral::new(
            Point {
                x: 0.0,
                y: 5.0,
            },
            Point {
                x: 5.0,
                y: 5.0,
            },
            Point {
                x: 5.0,
                y: 0.0,
            },
            Point {
                x: 0.0,
                y: 0.0,
            }
        );

    println!("My Line:");
    my_line.details();
    println!("Distance: {}", my_line.distance());

    println!("QuadrilateralLine:");
    let square_line = my_square.get_side_line(0);
    square_line.details();
    println!("Distance: {}", square_line.distance());

    my_line.translate_x(5.0);
    println!("My Line Translated:");
    my_line.details();

    println!("Negative values now");
    my_line.translate_x(-10.0);
    my_line.translate_y(-5.0);
    my_line.details();

    println!("Reflect on x axis");
    my_line.reflect_x();
    my_line.details();

    println!("Reflect on y axis");
    my_line.reflect_y();
    my_line.details();
}
