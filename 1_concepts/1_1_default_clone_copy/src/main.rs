
#[derive(Debug,Copy,Clone)]
pub struct Point{
    x: i64,
    y: i64,
}

impl Default for Point{
    fn default() -> Self {
        Point{
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Polyline{
    points: Vec<Point>,
}

impl Polyline{
    pub fn new(first_point: Point) -> Self{
        Polyline{
            points: vec![first_point],
        }
    }
    pub fn add_point(&mut self,point: Point){
        self.points.push(point);
    }
}




fn main() {
    let p1 = Point::default();
    let p2 = Point { x: 1, y: 2 };
    let mut polyline = Polyline::new(p1);
    polyline.add_point(p2);
    println!("{:?}",polyline);
}
//savonaarola check