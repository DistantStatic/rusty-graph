#[derive(Copy, Clone, Debug)]
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
    fn flip(self: &mut Self) {
        let old_point = self.points[0];
        self.points[0] = self.points[1];
        self.points[1] = old_point;
    }
}

struct Square {
    points: [Point; 4],
}

impl Square {
    fn get_single_line(self: &Self, side: usize) -> Line {
        let start = side % 4;
        let end = (start + 1) % 4;

        Line {
            points: [
                Point{
                    x: self.points[start].x,
                    y: self.points[start].y,
                },
                Point{
                    x: self.points[end].x,
                    y: self.points[end].y,
                }
            ]
        }
    }
}
        
fn main() {
    let my_line = Line {
        points: [
            Point{x: 0.0, y: 1.0},
            Point{x: 3.0,y: 4.0}
        ],
    };

    let my_square = Square {
        points: [
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
        ]
    };

    println!("My Line:");
    my_line.list_points();
    println!("Distance: {}", my_line.distance());

    println!("Square Line:");
    let square_line = my_square.get_single_line(0);
    square_line.list_points();
    println!("Distance: {}", square_line.distance());
}
