use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use rayon::prelude::*;

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    /// Returns the distance this point is from the origin
    ///
    /// `sqrt( x^2 + y^2 )`
    pub fn dist(&self) -> f32 {
        (self.x.powf(2_f32) + self.y.powf(2_f32)).sqrt()
    }
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let x = rng.gen();
        let y = rng.gen();

        Point { x, y }
    }
}

fn main() {
    let num_points: u64 = 10_000_000_000;
    let within_one = (0..num_points)
        .into_par_iter()
        .map(|_| {
            let mut rng = thread_rng();
            rng.gen::<Point>()
        })
        .filter(|point| point.dist() <= 1.0)
        .count();

    let pi = ((within_one as f64) / (num_points as f64)) * 4.0;

    println!("pi: {}", pi);
}
