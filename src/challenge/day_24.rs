pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    const MIN: f64 = 200000000000000f64;
    const MAX: f64 = 400000000000000f64;

    let lines = input
        .iter()
        .map(|line| {
            let mut iter = split_line(line);
            let cx = parse_number(iter.next().unwrap());
            let cy = parse_number(iter.next().unwrap());
            iter.next(); // skip z
            let dx = parse_number(iter.next().unwrap());
            let dy = parse_number(iter.next().unwrap());

            let d = dy / dx;
            let c = cy - d * cx;

            (c, d, cx, dx)
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for (i, &(c0, d0, cx0, dx0)) in lines.iter().take(lines.len() - 1).enumerate() {
        for &(c1, d1, cx1, dx1) in lines.iter().skip(i + 1) {
            let d = d0 - d1;

            if d == 0f64 {
                continue;
            }

            let x = (c1 - c0) / d;

            if !(MIN..=MAX).contains(&x)
                || x.total_cmp(&cx0) != dx0.total_cmp(&0f64)
                || x.total_cmp(&cx1) != dx1.total_cmp(&0f64)
            {
                continue;
            }

            let y = x * d0 + c0;

            if (MIN..=MAX).contains(&y) {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let mut input = input.iter();
    let h: [Hailstone; 3] = std::array::from_fn(|_| Hailstone::new(input.next().unwrap()));

    // The rock and hailstones are represented by 3 equations, one for each dimension
    // The rock is represented by:
    // x_r = dx_r * t + cx_r
    // y_r = dy_r * t + cy_r
    // z_r = dz_r * t + cz_r
    //
    // And the hailstones are represented by:
    // x_i = dx_i * t + cx_i
    // y_i = dy_i * t + cy_i
    // z_i = dz_i * t + cz_i
    //
    // The rock has to collide with each hailstone. The time of collisation is different for each hailstone.
    // (The rock might hit multiple hailstones at once though.)
    // This means that their coordinates have to match at one specific time, so:
    // dx_r * t_i + cx_r = dx_i * t_i + cx_i
    // dy_r * t_i + cy_r = dy_i * t_i + cy_i
    // dz_r * t_i + cz_r = dz_i * t_i + cz_i
    //
    // The time variable (t_i) is hard to work with, so let's eliminate it:
    // dx_r * t_i + cx_r = dx_i * t_i + cx_i
    // dx_r * t_i - dx_i * t_i = cx_i - cx_r  ; move termps with t_i to the left, and the rest to the right
    // t_i * (dx_r - dx_i) = cx_i - cx_r      ; extract t_i
    // t_i = (cx_i - cx_r) / (dx_r - dx_i)    ; solve for t_i
    //
    // The process can be repeated for all 3 dimensions
    // t_i = (cx_i - cx_r) / (dx_r - dx_i)
    // t_i = (cy_i - cy_r) / (dy_r - dy_i)
    // t_i = (cz_i - cz_r) / (dz_r - dz_i)
    //
    // The t_i part is the same for all 3 equations, so we can equate them 2 by 2 to get new equations (x/y example):
    // (cx_i - cx_r) / (dx_r - dx_i) = (cy_i - cy_r) / (dy_r - dy_i)
    // (cx_i - cx_r) * (dy_r - dy_i) = (cy_i - cy_r) * (dx_r - dx_i)  ; multiply both sides by the denominators
    //
    // By moving all factors containing only rock terms to the left:
    // cy_r * dx_r - cx_r * dy_r = cx_i * dy_i - cx_i * dy_r + cy_i * dx_r - cy_i * dx_i + cy_r * dx_i - cx_r * dy_i
    //
    // This is the x/y equation, we can get similar equations for x/z, and y/z:
    // cz_r * dx_r - cx_r * dz_r = cx_i * dz_i - cx_i * dz_r + cz_i * dx_r - cz_i * dx_i + cz_r * dx_i - cx_r * dz_i
    // cz_r * dy_r - cy_r * dz_r = cy_i * dz_i - cy_i * dz_r + cz_i * dy_r - cz_i * dy_i + cz_r * dy_i - cy_r * dz_i
    //
    // The resulting left hand side no longer depends on the hailstone values, so it's the same for all hailstones
    // This means we can equate the right hand sides of these equations of different hailstones (i/j example):
    // cx_i * dy_i - cx_i * dy_r + cy_i * dx_r - cy_i * dx_i + cy_r * dx_i - cx_r * dy_i
    //  = cx_j * dy_j - cx_j * dy_r + cy_j * dx_r - cy_j * dx_j + cy_r * dx_j - cx_r * dy_j
    //
    // Moving all factors to the same side produces:
    // cx_i * dy_i - cx_i * dy_r + cy_i * dx_r - cy_i * dx_i + cy_r * dx_i - cx_r * dy_i - cx_j * dy_j + cx_j * dy_r - cy_j * dx_r + cy_j * dx_j - cy_r * dx_j + cx_r * dy_j = 0
    //
    // And combining the common rock terms produces:
    // cx_r * (dy_j - dy_i) + cy_r * (dx_i - dx_j) + dx_r * (cy_i - cy_j) + dy_r * (cx_j - cx_i) + cx_i * dy_i - cx_j * dy_j + cy_j * dx_j - cy_i * dx_i = 0
    //
    // Moving the factors that don't rely on the rock (known values) to the right produces:
    // cx_r * (dy_j - dy_i) + cy_r * (dx_i - dx_j) + dx_r * (cy_i - cy_j) + dy_r * (cx_j - cx_i)
    //  = cx_j * dy_j - cx_i * dy_i + cy_i * dx_i - cy_j * dx_j
    //
    // The resulting equation is linear for the rock (each factor only contains 1 unknown term)
    // This means that these equations (x/y can be replaced with x/z and y/z) can be used to form a
    // linear equation system with 6 "terms" (treating the known values as a constants).
    // Each equation only contains 4 of the variables (2 dimensions) so we need to use multiple combinations of them.
    // For instance we can create a linear system of equations using
    // - x, y with hailstones 0, 1
    // - x, y with hailstones 0, 2
    // - x, z with hailstones 0, 1
    // - x, z with hailstones 0, 2
    // - y, z with hailstones 0, 1
    // - y, z with hailstones 0, 2
    //
    // For bravity: dx_ij represents (dx_i - dx_j), cy_10 represents (cy_1 - cy_0), etc.
    // So the previous equation above becomes:
    // cx_r * dy_ji + cy_r * dx_ij + dx_r * cy_ij + dy_r * cx_ji = cx_j * dy_j - cx_i * dy_i + cy_i * dx_ - cy_j * dx_j
    //
    // Using this, lets form a system of linear equations (as described above):
    // cx_r*dy_10 + cy_r*dx_01 + dx_r*cy_01 + dy_r*cx_10 = cx_1*dy_1 - cx_0*dy_0 + cy_0*dx_0 - cy_1*dx_1
    // cx_r*dy_20 + cy_r*dx_02 + dx_r*cy_02 + dy_r*cx_20 = cx_2*dy_2 - cx_0*dy_0 + cy_0*dx_0 - cy_2*dx_2
    // cx_r*dz_10 + cz_r*dx_01 + dx_r*cz_01 + dz_r*cx_10 = cx_1*dz_1 - cx_0*dz_0 + cz_0*dx_0 - cz_1*dx_1
    // cx_r*dz_20 + cz_r*dx_02 + dx_r*cz_02 + dz_r*cx_20 = cx_2*dz_2 - cx_0*dz_0 + cz_0*dx_0 - cz_2*dx_2
    // cy_r*dz_10 + cz_r*dy_01 + dy_r*cz_01 + dz_r*cy_10 = cy_1*dz_1 - cy_0*dz_0 + cz_0*dy_0 - cz_1*dy_1
    // cy_r*dz_20 + cz_r*dy_02 + dy_r*cz_02 + dz_r*cy_20 = cy_2*dz_2 - cy_0*dz_0 + cz_0*dy_0 - cz_2*dy_2
    //
    // Lets's rewrite the right hand side using new variables a, b, c, d, e, f
    // cx_r*dy_10 + cy_r*dx_01 + dx_r*cy_01 + dy_r*cx_10 = a
    // cx_r*dy_20 + cy_r*dx_02 + dx_r*cy_02 + dy_r*cx_20 = b
    // cx_r*dz_10 + cz_r*dx_01 + dx_r*cz_01 + dz_r*cx_10 = c
    // cx_r*dz_20 + cz_r*dx_02 + dx_r*cz_02 + dz_r*cx_20 = d
    // cy_r*dz_10 + cz_r*dy_01 + dy_r*cz_01 + dz_r*cy_10 = e
    // cy_r*dz_20 + cz_r*dy_02 + dy_r*cz_02 + dz_r*cy_20 = f
    //
    // And add the missing terms:
    // cx_r*dy_10 + dx_r*cy_01 + cy_r*dx_01 + dy_r*cx_10 +          0 +          0 = a (E1)
    // cx_r*dy_20 + dx_r*cy_02 + cy_r*dx_02 + dy_r*cx_20 +          0 +          0 = b (E2)
    // cx_r*dz_10 + dx_r*cz_01 +          0 +          0 + cz_r*dx_01 + dz_r*cx_10 = c (E3)
    // cx_r*dz_20 + dx_r*cz_02 +          0 +          0 + cz_r*dx_02 + dz_r*cx_20 = d (E4)
    //          0 +          0 + cy_r*dz_10 + dy_r*cz_01 + cz_r*dy_01 + dz_r*cy_10 = e (E5)
    //          0 +          0 + cy_r*dz_20 + dy_r*cz_02 + cz_r*dy_02 + dz_r*cy_20 = f (E6)
    //
    // This set of equations can now be be solved (by hand or) using Gaussian elimination

    let mut matrix = [
        [
            h[1].dy - h[0].dy,
            h[0].cy - h[1].cy,
            h[0].dx - h[1].dx,
            h[1].cx - h[0].cx,
            0f64,
            0f64,
            h[1].cx * h[1].dy - h[0].cx * h[0].dy + h[0].cy * h[0].dx - h[1].cy * h[1].dx,
        ],
        [
            h[2].dy - h[0].dy,
            h[0].cy - h[2].cy,
            h[0].dx - h[2].dx,
            h[2].cx - h[0].cx,
            0f64,
            0f64,
            h[2].cx * h[2].dy - h[0].cx * h[0].dy + h[0].cy * h[0].dx - h[2].cy * h[2].dx,
        ],
        [
            h[1].dz - h[0].dz,
            h[0].cz - h[1].cz,
            0f64,
            0f64,
            h[0].dx - h[1].dx,
            h[1].cx - h[0].cx,
            h[1].cx * h[1].dz - h[0].cx * h[0].dz + h[0].cz * h[0].dx - h[1].cz * h[1].dx,
        ],
        [
            h[2].dz - h[0].dz,
            h[0].cz - h[2].cz,
            0f64,
            0f64,
            h[0].dx - h[2].dx,
            h[2].cx - h[0].cx,
            h[2].cx * h[2].dz - h[0].cx * h[0].dz + h[0].cz * h[0].dx - h[2].cz * h[2].dx,
        ],
        [
            0f64,
            0f64,
            h[1].dz - h[0].dz,
            h[0].cz - h[1].cz,
            h[0].dy - h[1].dy,
            h[1].cy - h[0].cy,
            h[1].cy * h[1].dz - h[0].cy * h[0].dz + h[0].cz * h[0].dy - h[1].cz * h[1].dy,
        ],
        [
            0f64,
            0f64,
            h[2].dz - h[0].dz,
            h[0].cz - h[2].cz,
            h[0].dy - h[2].dy,
            h[2].cy - h[0].cy,
            h[2].cy * h[2].dz - h[0].cy * h[0].dz + h[0].cz * h[0].dy - h[2].cz * h[2].dy,
        ],
    ];

    let solution = gaussian_elimination(&mut matrix);
    Ok(solution[0] as i64 + solution[2] as i64 + solution[4] as i64)
}

fn split_line(line: &str) -> impl Iterator<Item = &[u8]> + '_ {
    line.as_bytes()
        .split(|char| !(char.is_ascii_digit() || *char == b'-'))
        .filter(|part| !part.is_empty())
}

fn parse_number(value: &[u8]) -> f64 {
    if value[0] == b'-' {
        -parse_number(&value[1..])
    } else {
        value
            .iter()
            .fold(0, |acc, char| acc * 10 + (char - b'0') as i64) as f64
    }
}

struct Hailstone {
    cx: f64,
    cy: f64,
    cz: f64,
    dx: f64,
    dy: f64,
    dz: f64,
}

impl Hailstone {
    fn new(line: &str) -> Self {
        let mut iter = split_line(line);

        Self {
            cx: parse_number(iter.next().unwrap()),
            cy: parse_number(iter.next().unwrap()),
            cz: parse_number(iter.next().unwrap()),
            dx: parse_number(iter.next().unwrap()),
            dy: parse_number(iter.next().unwrap()),
            dz: parse_number(iter.next().unwrap()),
        }
    }
}

const N: usize = 6;

fn gaussian_elimination(matrix: &mut [[f64; N + 1]; N]) -> [f64; N] {
    for i in 0..N {
        for j in i + 1..N {
            let ratio = matrix[j][i] / matrix[i][i];

            for k in 0..N + 1 {
                matrix[j][k] -= ratio * matrix[i][k];
            }
        }
    }

    let mut solution = [0f64; N];
    solution[N - 1] = (matrix[N - 1][N] / matrix[N - 1][N - 1]).round();

    for i in (0..N - 1).rev() {
        solution[i] = matrix[i][N];

        for j in i + 1..N {
            solution[i] -= matrix[i][j] * solution[j];
        }

        solution[i] = (solution[i] / matrix[i][i]).round();
    }

    solution
}
