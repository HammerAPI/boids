use std::f32::consts::FRAC_PI_4;

use ::rand::random;
use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, widgets},
};

const MAX_BOIDS: usize = 500;
const INITIAL_BOIDS: usize = 150;
const FRAC_3_PI_4: f32 = 3.0 * FRAC_PI_4;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Boid {
    id: usize,
    pos: Vec2,
    vel: Vec2,
    goal: Option<Vec2>,
    size: f32,
    flock: usize,
    max_speed: f32,
    min_speed: f32,
    separation_strength: f32,
    cohesion_strength: f32,
    alignment_strength: f32,
    goal_strength: f32,
}

impl Boid {
    fn new(id: usize, x: f32, y: f32) -> Self {
        let pos = Vec2::new(x, y);
        let vel = Vec2::new(random::<f32>() - 0.5, random::<f32>() - 0.5);
        let flock = random();
        Self {
            id,
            pos,
            vel,
            goal: None,
            size: 16.0,
            flock,
            max_speed: 4.0,
            min_speed: 1.0,
            separation_strength: 0.01,
            cohesion_strength: 0.001,
            alignment_strength: 0.1,
            goal_strength: 0.001,
        }
    }

    fn color(&self) -> Color {
        // let r = (self.dx() * self.max_speed * 255.0) as u8;
        // let g = (self.dy() * self.max_speed * 255.0) as u8;
        // let b = 0;

        // let r = self.id.wrapping_shl(0x158A634D) as u8;
        // let g = self.id.wrapping_shl(0x158A634D) as u8;
        // let b = self.id.wrapping_shl(0x158A634D) as u8;
        // Color::from_rgba(r, g, b, u8::MAX)
        Color::from_hex(self.flock as u32)
    }

    fn x(&self) -> f32 {
        self.pos.x
    }
    fn y(&self) -> f32 {
        self.pos.y
    }

    fn dx(&self) -> f32 {
        self.vel.x
    }
    fn dy(&self) -> f32 {
        self.vel.y
    }
    fn size(&self) -> f32 {
        self.size
    }
    fn rotation(&self) -> f32 {
        self.dy().atan2(self.dx())
    }

    fn fov(&self) -> f32 {
        self.size() * 5.0
    }

    fn in_sight(&self, other: &Self) -> bool {
        self.pos.distance_squared(other.pos) <= self.fov().powi(2)
    }

    fn too_close(&self, other: &Self) -> bool {
        self.pos.distance_squared(other.pos) <= self.size().powi(2)
    }

    fn calculate_new_velocity(&mut self, boids: &[Self]) {
        let mut separation = Vec2::default();
        let mut cohesion = Vec2::default();
        let mut alignment = Vec2::default();
        let mut goal = Vec2::default();

        let mut num_visible = 0;
        let mut num_too_close = 0;
        for other in boids {
            if self != other {
                if self.too_close(other) {
                    separation += self.pos - other.pos;
                    num_too_close += 1;
                }

                if self.in_sight(other) {
                    num_visible += 1;
                    cohesion += other.pos;
                    alignment += other.vel;
                }
            }
        }

        if num_too_close > 0 {
            separation /= num_too_close as f32;
        }

        if num_visible > 0 {
            cohesion /= num_visible as f32;
            alignment /= num_visible as f32;

            cohesion -= self.pos;
            alignment -= self.vel;
        }

        if let Some(g) = self.goal {
            goal = self.pos - g; // Repel

            // goal = g - self.pos; // Attract
        }

        self.vel += separation * self.separation_strength;
        self.vel += cohesion * self.cohesion_strength;
        self.vel += alignment * self.alignment_strength;
        self.vel += goal * self.goal_strength;
    }

    fn limit_speed(&mut self) {
        let speed = (self.dx().powi(2) + self.dy().powi(2)).sqrt();

        if speed > self.max_speed {
            self.vel = self.vel / speed * self.min_speed;
        }
        if speed < self.min_speed {
            self.vel = self.vel / speed * self.max_speed;
        }
    }

    fn update(&mut self) {
        self.limit_speed();
        self.pos += self.vel;
        wrap_edges(&mut self.pos);
    }
}

#[macroquad::main("Boids")]
async fn main() {
    let mut separation_strength = 0.01;

    let mut boids = Vec::with_capacity(MAX_BOIDS);

    for i in 0..INITIAL_BOIDS {
        let x = random::<f32>() * screen_width();
        let y = random::<f32>() * screen_height();
        let boid = Boid::new(i, x, y);
        boids.push(boid);
    }

    loop {
        clear_background(WHITE);

        // widgets::Window::new(0, vec2(400., 200.), vec2(320., 400.))
        //     .label("New Window")
        //     .titlebar(true)
        //     .ui(&mut *root_ui(), |ui| {
        //         ui.slider(0, "Separation Strength", 0.0..1.0, &mut separation_strength)
        //     });

        widgets::Group::new(hash!(), vec2(screen_width() / 2.0, screen_height() / 10.0)).ui(
            &mut root_ui(),
            |ui| {
                ui.label(None, "Separation");
                ui.slider(hash!(), "[0, 1.0]", 0.0..1.0, &mut separation_strength);
                ui.label(None, "Cohesion");
                ui.slider(hash!(), "[0, 1.0]", 0.0..1.0, &mut separation_strength);
            },
        );

        let (mouse_x, mouse_y) = mouse_position();
        // println!("Mouse: ({mouse_x}, {mouse_y})");
        if is_mouse_button_pressed(MouseButton::Left) {
            if boids.len() < MAX_BOIDS {
                println!("Spawning new Boid at ({mouse_x}, {mouse_y})");
                boids.push(Boid::new(boids.len(), mouse_x, mouse_y));
            } else {
                println!("Already at maximum number of boids ({MAX_BOIDS})");
            }
        }

        for i in 0..boids.len() {
            // Fetch the boid to modify
            let mut boid = boids[i];

            if is_mouse_button_down(MouseButton::Right) {
                boid.goal = Some(vec2(mouse_x, mouse_y));
            } else {
                boid.goal.take();
            }

            boid.calculate_new_velocity(&boids);

            boid.update();

            // Update the boid in the list
            boids[i] = boid;

            draw_boid(&boid);
        }

        next_frame().await
    }
}

fn draw_boid(boid: &Boid) {
    // Boid's FOV
    // draw_circle_lines(boid.x(), boid.y(), boid.fov(), screen_dpi_scale(), BLACK);

    let rot = boid.rotation();
    let (x, y) = (boid.x(), boid.y());
    let length = boid.size();

    let v1 = vec2(x + length * rot.cos(), y + length * rot.sin());
    let v2 = vec2(
        x + length * (rot + FRAC_3_PI_4).cos(),
        y + length * (rot + FRAC_3_PI_4).sin(),
    );
    let v3 = vec2(
        x + length * (rot - FRAC_3_PI_4).cos(),
        y + length * (rot - FRAC_3_PI_4).sin(),
    );

    draw_triangle(v1, v2, v3, boid.color());
}

fn wrap_edges(v: &mut Vec2) {
    if v.x > screen_width() {
        v.x = 0.0;
    }
    if v.x < 0.0 {
        v.x = screen_width()
    }
    if v.y > screen_height() {
        v.y = 0.0;
    }
    if v.y < 0.0 {
        v.y = screen_height()
    }
}
