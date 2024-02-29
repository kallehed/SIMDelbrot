use std::io::Write;

type F = f64;

#[derive(Clone, Copy)]
struct Pos {
    x: F,
    y: F,
}
const HEIGHT: i64 = 70;
const WIDTH: i64 = 152;

// WIDTH has to be divisible by 4
const _: [(); 0 - (WIDTH % 4 != 0) as usize] = [];

fn main() {
    // let goto = Pos{x: 0.42884,y: -0.231345};
    let goto = Pos {
        x: -1.629170093905343,
        y: -0.0203968,
    };

    let palette = br##"`.-':_,^"=;><+!r~c*/\z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@"##;

    // let goto = Pos{x: -0.761574, y:-0.0847596};
    let mut out = std::io::stdout().lock();
    let mut print_buffer = [b'\n'; (WIDTH as usize + 1) * HEIGHT as usize];

    // 78
    for zoom in 1.. {
        let size = 10.0 * (1.03 as F).powf(-zoom as F);
        println!("size: {}", size);

        let mut most_bounces = 0;
        for y in 0..HEIGHT {
            for x in (0..WIDTH).step_by(4) {
                for (idx, b) in unsafe {
                    in_mandelbrot_set_4([(x, y), (x + 1, y), (x + 2, y), (x + 3, y)].map(
                        |(x, y)| Pos {
                            x: (((x as F / WIDTH as F) - 0.5) * 2.0) * size + goto.x,
                            y: (((y as F / HEIGHT as F) - 0.5) * 2.0) * size + goto.y,
                        },
                    ))
                }
                .into_iter()
                .enumerate()
                {
                    print_buffer[(y * (WIDTH + 1)) as usize + (x as usize + idx)] = match b {
                        0 => b' ',
                        bounces => {
                            most_bounces = most_bounces.max(bounces);
                            let ch = ((bounces - 2) % palette.len() as i32) as u8;
                            palette[ch as usize]
                        }
                    };
                }
            }
        }
        out.write(&print_buffer).unwrap();
        // println!("max bounces: {}", most_bounces);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

// fn in_mandelbrot_set(c: Pos) -> i32 {
//     let mut z = Pos { x: 0.0, y: 0.0 };
//     for bounce in 1..500 {
//         let zx_abs = z.x * z.x;
//         let zy_abs = z.y * z.y;
//         z = Pos {
//             x: zx_abs - zy_abs + c.x,
//             y: 2.0 * z.x * z.y + c.y,
//         };
//         if zx_abs + zy_abs >= 4.0 {
//             // bc of both sides squared
//             return bounce;
//         }
//     }
//     0
// }

#[target_feature(enable = "avx2")]
#[target_feature(enable = "sse4.2")]
#[target_feature(enable = "fma")]
unsafe fn in_mandelbrot_set_4(cs: [Pos; 4]) -> [i32; 4] {
    use std::arch::x86_64::*;
    let twos = _mm256_set1_pd(2.0);
    let fours = _mm256_set1_pd(4.0);
    let cx: __m256d = _mm256_set_pd(cs[3].x, cs[2].x, cs[1].x, cs[0].x);
    let cy: __m256d = _mm256_set_pd(cs[3].y, cs[2].y, cs[1].y, cs[0].y);

    let mut zx = _mm256_setzero_pd();
    let mut zy = _mm256_setzero_pd();

    let mut bounces_to_leave = [0, 0, 0, 0];
    let mut left = 0;

    for bounce in 1..300 {
        let zx_abs = _mm256_mul_pd(zx, zx);
        let zy_abs = _mm256_mul_pd(zy, zy);
        let new_zx = _mm256_add_pd(_mm256_sub_pd(zx_abs, zy_abs), cx);
        let new_zy = _mm256_add_pd(_mm256_mul_pd(_mm256_mul_pd(twos, zx), zy), cy);
        zx = new_zx;
        zy = new_zy;
        let abs_squared = _mm256_add_pd(zx_abs, zy_abs);
        let beyond = _mm256_cmp_pd::<_CMP_LE_OS>(fours, abs_squared);
        let beyond = _mm256_castpd_si256(beyond);
        let beyond = [
            _mm256_extract_epi64::<0>(beyond),
            _mm256_extract_epi64::<1>(beyond),
            _mm256_extract_epi64::<2>(beyond),
            _mm256_extract_epi64::<3>(beyond),
        ]
        .map(|x| x == -1);
        for i in 0..4 {
            if bounces_to_leave[i] == 0 {
                if beyond[i] {
                    bounces_to_leave[i] = bounce;
                    left += 1;
                    if left == 4 {
                        return bounces_to_leave;
                    }
                }
            }
        }
    }
    bounces_to_leave
}
