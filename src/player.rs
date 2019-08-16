use crate::enemy::Enemy;
use ggez::graphics::Mesh;
use ggez::*;

pub struct Player {
    pub hp: u32,
    pub x: f32,
    pub y: f32,
    point: nalgebra::Point2<f32>,
    pub bullets: Vec<Bullet>,
    pub hitbox: graphics::Rect,
}

impl Player {
    pub fn new(xpos: f32, ypos: f32) -> Player {
        Player {
            hp: 10,
            x: xpos,
            y: ypos,
            point: nalgebra::Point2::new(0.0, 0.0),
            bullets: vec![],
            hitbox: graphics::Rect::new(xpos, ypos, 50.0, 50.0),
        }
    }

    pub fn update(&mut self, ctx: &mut Context, xpos: f32, ypos: f32) {
        self.point.x = xpos;
        self.point.y = ypos;

        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Space) {
            self.bullets.push(Bullet::new(self.x, self.y, xpos, ypos));
        }

        self.bullets.iter_mut().for_each(|b| {
            b.update(ctx);
        });
        self.bullets.retain(|b| b.isalive());
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mesh: Mesh = graphics::MeshBuilder::new()
            .rectangle(graphics::DrawMode::fill(), self.hitbox, graphics::WHITE)
            .build(ctx)
            .expect("Failed to draw player");

        let my_dest = nalgebra::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(my_dest)).expect("");

        self.bullets.iter_mut().for_each(|b| {
            b.draw(ctx);
        });
    }

    pub fn update_hp(&mut self, enemies: &[Enemy]) {
        enemies.iter().for_each(|e| {
            if e.hitbox.overlaps(&self.hitbox) && self.hp != 0 {
                self.hp -= e.dmg;
            }
        });
    }

    pub fn isalive(&self) -> bool {
        self.hp > 0
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
            hitbox: graphics::Rect::new(xpos, ypos, 10.0, 10.0),
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        let x = self.x;
        let y = self.y;

        if self.x >= self.dest_x {
            self.x -= 10.0;
        }
        if self.x <= self.dest_x {
            self.x += 10.0;
        }

        if self.y >= self.dest_y {
            self.y -= 10.0;
        }
        if self.y <= self.dest_y {
            self.y += 10.0;
        }

        self.hitbox.x = self.x;
        self.hitbox.y = self.y;

        if self.x == x {
            self.dead_x = true;
        } else {
            self.dead_x = false;
        }
        if self.y == y {
            self.dead_y = true;
        } else {
            self.dead_y = false;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let mesh: Mesh = graphics::MeshBuilder::new()
            .rectangle(graphics::DrawMode::fill(), self.hitbox, graphics::WHITE)
            .build(ctx)
            .expect("Failed to draw bullet");

        let my_dest = nalgebra::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &mesh, graphics::DrawParam::default().dest(my_dest)).expect("");
    }

    pub fn isalive(&self) -> bool {
        if self.dead_x && self.dead_y {
            return false;
        }
        true
    }
}
