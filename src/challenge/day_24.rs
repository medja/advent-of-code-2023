use z3::ast::Ast;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    const MIN: f64 = 200000000000000f64;
    const MAX: f64 = 400000000000000f64;

    let lines = input
        .iter()
        .map(|line| {
            let mut iter = parse_values(line);
            let cx = iter.next().unwrap() as f64;
            let cy = iter.next().unwrap() as f64;
            iter.next(); // skip z
            let dx = iter.next().unwrap() as f64;
            let dy = iter.next().unwrap() as f64;

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
    let config = z3::Config::default();
    let context = z3::Context::new(&config);
    let solver = z3::Solver::new(&context);

    let xr = z3::ast::Int::new_const(&context, "x");
    let yr = z3::ast::Int::new_const(&context, "y");
    let zr = z3::ast::Int::new_const(&context, "z");

    let dxr = z3::ast::Int::new_const(&context, "dx");
    let dyr = z3::ast::Int::new_const(&context, "dy");
    let dzr = z3::ast::Int::new_const(&context, "dz");

    let zero = z3::ast::Int::from_i64(&context, 0);

    for (i, line) in input.iter().enumerate().take(3) {
        let mut iter = parse_values(line);
        let x = z3::ast::Int::from_i64(&context, iter.next().unwrap());
        let y = z3::ast::Int::from_i64(&context, iter.next().unwrap());
        let z = z3::ast::Int::from_i64(&context, iter.next().unwrap());
        let dx = z3::ast::Int::from_i64(&context, iter.next().unwrap());
        let dy = z3::ast::Int::from_i64(&context, iter.next().unwrap());
        let dz = z3::ast::Int::from_i64(&context, iter.next().unwrap());
        let t = z3::ast::Int::new_const(&context, i as u32);

        solver.assert(&(&dxr * &t + &xr)._eq(&(dx * &t + x)));
        solver.assert(&(&dyr * &t + &yr)._eq(&(dy * &t + y)));
        solver.assert(&(&dzr * &t + &zr)._eq(&(dz * &t + z)));
        solver.assert(&t.gt(&zero));
    }

    if solver.check() != z3::SatResult::Sat {
        anyhow::bail!("Cannot solve system or equations");
    }

    let model = solver.get_model().unwrap();
    let result = model.eval(&(xr + yr + zr), false).unwrap();

    Ok(result.as_i64().unwrap())
}

fn parse_values(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.as_bytes()
        .split(|char| !(char.is_ascii_digit() || *char == b'-'))
        .filter(|part| !part.is_empty())
        .map(parse_number)
}

fn parse_number(value: &[u8]) -> i64 {
    if value[0] == b'-' {
        -parse_number(&value[1..])
    } else {
        value
            .iter()
            .fold(0, |acc, char| acc * 10 + (char - b'0') as i64)
    }
}
