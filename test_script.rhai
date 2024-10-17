let r = extern(20.0, 200.0);
let r_min = extern(0.1, 10.0);
let r_max = extern(20.0, 100.0);
let t = extern(1.0, 10.0);
let n = extern(5, 10);

let canvas = canvas_width_height(400, 300);
let stroke = stroke(t, BLACK);
let step = 2 * pi() / n;

for i in 0..n {
	let radius = LINEAR.evaluate(r_min, r_max, to_float(i) / n);
	let pt = point_polar(r, i * step);
	let circle = circle_at(radius, pt.x, pt.y);
	canvas.draw(circle, stroke);
}

canvas

