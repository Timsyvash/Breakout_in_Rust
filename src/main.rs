mod Game;
use Game::*;
use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Арканоїд", [400, 300])
            .exit_on_esc(true)
            .build().unwrap();

    let mut blocks = Vec::new();

    for row in 0..3 {
        for col in 0..8 {
            blocks.push(Blocks {
                pos_x: 5.0 + col as f64 * 50.0,
                pos_y: 5.0 + row as f64 * 25.0,
                is_exists: true,
                width: 40.0,
                height: 15.0,
            });
        }
    }

    let bs = vec![
        Blocks {
            pos_x: 5.0,
            pos_y: 5.0,
            is_exists: true,
            width: 40.0,
            height: 15.0,
        }
    ];

    let mut game = Game::Game {
        pause: false,
        pos_x: 300.0 / 2.0 + 30.0,
        pos_y: 285.0,
        ball_x: 200.0,
        ball_y: 250.0,
        ball_dx: 2.0,
        ball_dy: -2.0,
        width_r: 60.0,
        height_r: 15.0,
        ball_width: 10.0,
        ball_height: 10.0,
        blocks,
    };

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.move_keys(key);
        }

        if let Some(_) = e.update_args() {
            game.game_update();
        }

        window.draw_2d(&e, |c, g| {
            game.render(c, g);
        });
    }
}
