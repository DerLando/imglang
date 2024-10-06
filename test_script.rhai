let r = extern(50.0, 200.0);
let t = extern(1.0, 10.0);
let n = extern(5, 10);

// let r = 50.0;
// let t = 3.0;
// let n = 7;

let canvas = canvas_width_height(400, 300);
let stroke = stroke(t, BLACK);
let circle = circle(r);

for i in 0..n {
	let xform = translation(i * r * 2.0, 0.0);
	let current = transform(circle, xform);
	canvas.draw(current, stroke);
}


canvas

