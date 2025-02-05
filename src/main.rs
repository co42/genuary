use std::fs;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Genuary".to_owned(),
        window_width: 860,
        window_height: 860,
        window_resizable: false,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let sw = screen_width();
    let sh = screen_height();
    assert_eq!(sw, sh);
    let mut seed = 1;
    let record = true;
    if record {
        fs::create_dir_all("video/day-01/frames").expect("Clear frames");
        fs::remove_dir_all("video/day-01/frames").expect("Clear frames");
        fs::create_dir_all("video/day-01/frames").expect("Clear frames");
    }

    let mut ctx = Context {
        ma: 1.0,
        w: 2.0,
        f: 0,
        t: 0,
        ts: 10,
        tb: 100,
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Right) {
            seed += 1;
            ctx.t = 0;
        }
        if is_key_pressed(KeyCode::Left) {
            seed -= 1;
            ctx.t = 0;
        }
        if is_key_pressed(KeyCode::Up) {
            ctx.ts += 1;
            println!("Set ts={}", ctx.ts);
        }
        if is_key_pressed(KeyCode::Down) {
            ctx.ts = (ctx.ts - 1).max(1);
            println!("Set ts={}", ctx.ts);
        }
        // Next animation cycle
        if ctx.t >= 2 * ctx.tb {
            ctx.t = 0;
            seed += 1;
        }

        clear_background(Color::from_hex(0x110000));

        fastrand::seed(seed);
        draw_square(&ctx, Vec2::new(0.0, 0.0), Vec2::new(sw, sh), 0);

        if record {
            get_screen_data().export_png(&format!("video/day-01/frames/{:05}.png", ctx.f));
            if seed > 5 {
                break;
            }
        }

        ctx.t += 1;
        ctx.f += 1;
        next_frame().await;
    }

    if record {
        let mut cmd = std::process::Command::new("ffmpeg");
        cmd.args(&[
            "-y",
            "-framerate",
            "60",
            "-i",
            "video/day-01/frames/%05d.png",
            "-c:v",
            "libx264",
            "video/day-01/output.mp4",
        ]);
        cmd.stdout(std::process::Stdio::null());
        cmd.stderr(std::process::Stdio::null());
        cmd.output().expect("Run ffmpeg");
    }
}

struct Context {
    ma: f32,
    w: f32,
    f: u32,
    t: u32,
    ts: u32,
    tb: u32,
}

fn draw_square(ctx: &Context, mut start: Vec2, mut size: Vec2, d: u32) {
    // Remove margin
    start += ctx.ma;
    size -= 2.0 * ctx.ma;
    if size.x <= ctx.w {
        return;
    }

    // Filter some squares
    if d > 0 && fastrand::f32() < 0.25 {
        return;
    }

    if d > 0 {
        let r = fastrand::f32();
        let color = if r < 0.3 {
            Color::from_hex(0xF50057)
        } else if r < 0.6 {
            Color::from_hex(0xFF1744)
        } else {
            Color::from_hex(0xFF4081)
        };

        // Animate square
        let t = ctx.t % (2 * ctx.tb);
        let dts = d * ctx.ts - fastrand::u32(0..ctx.ts);
        let draw = if t < ctx.tb {
            // Appears
            t > dts
        } else {
            // Disappears
            t - ctx.tb < dts
        };

        if draw {
            draw_rectangle_lines(start.x, start.y, size.x, size.y, ctx.w, color);
        }
    }

    // Recurse
    start += ctx.w;
    size -= 2.0 * ctx.w;
    let hsize = size / 2.0;
    draw_square(ctx, start, hsize, d + 1);
    draw_square(ctx, start + hsize, hsize, d + 1);
    draw_square(ctx, start.with_x(start.x + hsize.x), hsize, d + 1);
    draw_square(ctx, start.with_y(start.y + hsize.y), hsize, d + 1);
}
