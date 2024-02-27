use std::io::Write;

type F = f64;

#[derive(Clone, Copy)]
struct Pos {
    x: F,
    y: F,
}

fn main() {
    println!("Hello, world!");

    // let goto = Pos{x: 0.42884,y: -0.231345};
    let goto = Pos{x: -1.629170093905343, y: -0.0203968};
    // let goto = Pos{x: -0.761574, y:-0.0847596};
    let mut out = std::io::stdout().lock();

    // 78
    for zoom in 1.. {
        let size = (1.05 as F).powf(-zoom as F);
        println!("size: {}", size);

        let height = 70;
        let width = 150;
        let mut most_bounces = 0;
        for y in 0..height {
            for x in 0..width {
                match in_mandelbrot_set(Pos {
                    x: (((x as F / width as F) - 0.5) * 2.0) * size + goto.x,
                    y: (((y as F / height as F) - 0.5) * 2.0) * size + goto.y,
                }) {
                    0 => {out.write(&[b' ']).unwrap();},
                    x => {
                        most_bounces = most_bounces.max(x);
                        // let x = ((x - 1)/1) as u8;
                        // let x = x.saturating_add(33);
                        // let x = x.min(126);
                        let x = (x % (126 - 33 + 1)) as u8;
                        let x = x + 33;
                        let x = x.min(126);
                        out.write(&[x]).unwrap();
                    },
                }
            }
            println!();
        }
        println!("max bounces: {}", most_bounces);
        std::thread::sleep(std::time::Duration::from_millis(40));
    }
}

fn in_mandelbrot_set(c: Pos) -> i32 {
    fn f(z: Pos, c: Pos) -> Pos {
        Pos {
            x: z.x * z.x - z.y * z.y + c.x,
            y: 2.0 * z.x * z.y + c.y,
        }
    }
    let mut z = Pos { x: 0.0, y: 0.0 };
    for bounce in 1..500 {
        z = f(z, c);
        if z.x * z.x + z.y * z.y >= 4.0 {
            return bounce;
        }
    }
    0
}
