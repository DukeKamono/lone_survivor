use ggez::*;
use std::time::Duration;

mod player;
use player::Player;

mod enemy;
use enemy::Enemy;

struct State {
    player: Player,
    enemies: Vec<Enemy>,
    dt: Duration,
    spawn: Duration,
    score: u32,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let delta = timer::delta(ctx);
        
        let m = input::mouse::position(ctx);
        
        if self.dt > Duration::from_millis(10u64) {
            self.player.update(ctx, m.x, m.y);
            self.dt = Duration::new(0, 0);
        }
        else {
            self.dt += delta;
        }


        if self.spawn > Duration::from_millis(1000u64) {
            self.enemies.push(Enemy::new());
            self.spawn = Duration::new(0, 0);

            let px = self.player.x;
            let py = self.player.y;
            let bull = &self.player.bullets;
            let mut s = self.score;

            self.enemies.iter_mut().for_each(|e| {
                e.update_hp(bull);
                if !e.isalive() {
                    s += 1;
                    println!("{}", s);
                }
            });

            self.score += s;
            s = 0;

            self.enemies.retain(|e| e.isalive());
            self.enemies.iter_mut().for_each(|e| {
                e.update(ctx, px, py);
            });
        }
        else {
            self.spawn += delta;
        }
        
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.player.draw(ctx);

        self.enemies.iter_mut().for_each(|e| {
            e.draw(ctx);
        });

        //ui
        graphics::draw_queued_text(
            ctx,
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("unable to clear queue");

        let points = nalgebra::Point2::new(100.0, 10.0);
        graphics::queue_text(
            ctx,
            &graphics::Text::new((self.score.to_string(), graphics::Font::default(), 22.0)),
            points,
            Some(graphics::WHITE),
        );



        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let p = Player::new(400.0, 250.0);
    let e = vec!();

    let state = &mut State {
        player: p,
        enemies: e,
        score: 0,
        dt: Duration::new(0, 0),
        spawn: Duration::new(0, 0),
    };

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("lone_survivor", "James T. Moore")
    .conf(c)
    .build()
    .unwrap();
    
    event::run(ctx, event_loop, state).unwrap();
}
