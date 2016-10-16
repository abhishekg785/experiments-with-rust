//strcuture has a basic feel gf classes
// to have the functions along with strcuture we can use impl  just like classes

struct Circle {
	x:f64,
	y:f64,
	radius:f64
}

impl Circle {
	fn area(&self) -> f64 {
		std::f64::consts::PI * (self.radius * self.radius)
	}

	fn perimter(&self) -> f64 {
		std::f64::consts::PI * (self.radius) * 2.0
	}
}

fn main() {
	let c = Circle { x: 0.0, y: 0.0, radius:2.0 };
	println!("Area : {}", c.area());
	println!("Perimeter : {}", c.perimter());
}