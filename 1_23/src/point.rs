#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, p: &Self) -> f64 {
        let dx = (self.x - p.x).abs();
        let dy = (self.y - p.y).abs();
        (dx * dx + dy * dy).sqrt()
    }
}