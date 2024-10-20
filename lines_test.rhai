let r = extern(20.0, 200.0);
let t = extern(1.0, 10.0);

let canvas = canvas_width_height(400, 400);
let stroke = stroke(t, BLACK);
let frame = to_float(TIME % 1000);

let ground = line(-200.0, 0.0, 200.0, 0.0);
canvas.draw(ground, stroke(t * 2.0, BLACK));

let height = (sin(frame / 20.0) + 5.0) * 10.0;
let left_bottom = point(-100.0, 0.0);
let left_top = point(-50.0, height);
let slope_start = line(left_bottom, left_top);
canvas.draw(slope_start, stroke); 

let right_bottom = point(100.0, 0.0);
let right_top = point(50.0, height);
canvas.draw(line(right_bottom, right_top), stroke);
canvas.draw(line(left_top, right_top), stroke);

canvas