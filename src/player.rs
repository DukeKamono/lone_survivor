use ggez::*;
use ggez::graphics::Mesh;

pub struct Player {
    pub x: f32,
    pub y: f32,
    point: nalgebra::Point2<f32>,
    pub bullets: Vec<Bullet>,
    pub hitbox: graphics::Rect,
}

impl Player {
    pub fn new(xpos: f32, ypos: f32) -> Player {
        Player {
            x: xpos,
            y: ypos,
            point: nalgebra::Point2::new(0.0, 0.0),
            bullets: vec!(),
            hitbox: graphics::Rect::new(xpos, ypos, 50.0, 50.0),
        }
    }
    
    pub fn update(&mut self, ctx: &mut Context, xpos: f32, ypos: f32) {
        self.point.x = xpos;
        self.point.y = ypos;

        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Space) {
            self.bullets.push(Bullet::new(self.x, self.y, xpos, ypos));
        }
        
        self.bullets.retain(|b| b.isalive());

        self.bullets.iter_mut().for_each(|b| {
            b.update(ctx);
        });
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mesh: Mesh = graphics::MeshBuilder::new()
        .rectangle(graphics::DrawMode::fill(), self.hitbox, graphics::WHITE)
        .build(ctx).expect("Failed to draw player");

        let my_dest = nalgebra::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(my_dest)).expect("");

        self.bullets.iter_mut().for_each(|b| {
            b.draw(ctx);
        });
    }
}

pub struct Bullet {
    x: f32,
    y: f32,
    pub dmg: u32,
    dest_x: f32,
    dest_y: f32,
    dead_x: bool,
    dead_y: bool,
    slope: f32,
    slope_y: f32,
    slope_x: f32,
    pub hitbox: graphics::Rect,
}

impl Bullet {
    pub fn new(xpos: f32, ypos: f32, dx: f32, dy: f32) -> Bullet {
        Bullet {
            x: xpos + 25.0,
            y: ypos + 25.0,
            dmg: 1,
            dest_x: dx,
            dest_y: dy,
            dead_x: false,
            dead_y: false,
            slope: ((dy - ypos + 25.0) / (dx - xpos + 25.0)),
            slope_y: dy - ypos,
            slope_x: dx - xpos,
            hitbox: graphics::Rect::new(xpos, ypos, 10.0, 10.0),
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        //println!("{} {}", self.dest_x, self.dest_y);

        //let slope = (self.dest_y - self.y) / (self.dest_x - self.x);
        //let slope_y = self.dest_y - self.y;
        //let slope_x = self.dest_x - self.x;

        //println!("{} {} {}", slope, slope_y, slope_x);

        //if self.x < self.dest_x {
        //    self.x += 10.0;
        //    if self.x > self.dest_x {
        //        self.dead_x = true;
        //    }
        //}
        //else {
        //    self.x -= 10.0;
        //    if self.x < self.dest_x {
        //        self.dead_x = true;
        //    }
        //}
        //
        //if self.y < self.dest_y {
        //    self.y += 10.0;
        //    if self.y > self.dest_y {
        //        self.dead_y = true;
        //    }
        //}
        //else {
        //    self.y -= 10.0;
        //    if self.y < self.dest_y {
        //        self.dead_y = true;
        //    }
        //}
        
        // top right quadrant
        if self.slope_x > 0.0 && self.slope_y > 0.0 {
            if self.x < self.dest_x && self.y < self.dest_y {
                self.x += self.slope;
                self.y += self.slope;
            }
        }
        // top left quadrant
        if self.slope_x < 0.0 && self.slope_y > 0.0 {
            if self.x > self.dest_x && self.y < self.dest_y {
                self.x += self.slope;
                self.y += self.slope;
            }
        }

        // bottom left quadrant
        if self.slope_x < 0.0 && self.slope_y < 0.0 {
            if self.x > self.dest_x && self.y > self.dest_y {
                self.x += self.slope;
                self.y += self.slope;
            }
        }

        // bottom right quadrant
        if self.slope_x > 0.0 && self.slope_y < 0.0 {
            if self.x < self.dest_x && self.y > self.dest_y {
                self.x += self.slope;
                self.y += self.slope;
            }
        }

        self.hitbox.x = self.x;
        self.hitbox.y = self.y;
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mesh: Mesh = graphics::MeshBuilder::new()
        .rectangle(graphics::DrawMode::fill(), self.hitbox, graphics::WHITE)
        .build(ctx).expect("Failed to draw bullet");

        let my_dest = nalgebra::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(my_dest)).expect("");
    }

    pub fn isalive(&self) -> bool {
        !self.dead_x && !self.dead_y
    }
}