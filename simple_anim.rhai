let r = extern(20.0, 200.0);
let t = extern(1.0, 10.0);

let canvas = canvas_width_height(400, 400);
let stroke = stroke(t, BLACK);
let frame = to_float(TIME % 1000);

let radius = (sin(frame / 20.0) + 2.0) * 40.0;
let circle = circle(radius);
canvas.draw(circle, stroke);

let dist = radius + r;
let angle = frame / 20;
let center = point_polar(dist, angle);
let circle = circle_at(r, center.x, center.y);
canvas.draw(circle, stroke);

canvas