pub fn torus(row: u16, column: u16, i_rad: f32, o_rad: f32) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<u16>) {
    let mut pos = Vec::new();
    let mut nor = Vec::new();
    let mut col = Vec::new();
    let mut idx = Vec::new();
    const PI: f32 = std::f32::consts::PI;

    for i in 0..=row {
        let r = PI * 2. / row as f32 * i as f32;
        let rr = r.cos();
        let ry = r.sin();
        for ii in 0..=column {
            let tr = PI * 2. / column as f32 * ii as f32;
            let tx = (rr * i_rad + o_rad) * tr.cos();
            let ty = ry * i_rad;
            let tz = (rr * i_rad + o_rad) * tr.sin();
            let rx = rr * tr.cos();
            let rz = rr * tr.sin();
            pos.push(tx);
            pos.push(ty);
            pos.push(tz);
            nor.push(rx);
            nor.push(ry);
            nor.push(rz);
            let tc = hsva(360. / column as f32 * ii as f32, 1., 1., 1.).unwrap();
            for c in tc {
                col.push(c);
            }
        }
    }

    for i in 0..row {
        for ii in 0..column {
            let r = (column + 1) * i + ii;
            idx.push(r);
            idx.push(r + column + 1);
            idx.push(r + 1);
            idx.push(r + column + 1);
            idx.push(r + column + 2);
            idx.push(r + 1);
        }
    }

    (pos, nor, col, idx)
}

fn hsva(h: f32, s: f32, v: f32, a: f32) -> Result<[f32; 4], String> {
    if s > 1. || v > 1. || a > 1. {
        return Err("invalid value".to_string());
    }

    if s == 0. {
        return Ok([v, v, v, a]);
    }

    let th = h % 360.;
    let i = (th / 60.).floor() as usize;
    let f = th / 60. - i as f32;
    let m = v * (1. - s);
    let n = v * (1. - s * f);
    let k = v * (1. - s * (1. - f));

    let r = [v, n, m, m, k, v];
    let g = [k, v, v, n, m, m];
    let b = [m, m, k, v, v, n];

    Ok([r[i], g[i], b[i], a])
}