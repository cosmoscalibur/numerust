pub fn incremental(f: fn(f32) -> f32, start: f32, end: f32, delta: f32) -> Option<f32> {
    if start > end {
        return None;
    }
    if f(start) * f(end) > 0.0 {
        return None;
    }
    let mut x0 = start;
    let mut x1 = (x0 + delta).min(end);
    while x1 <= end && f(x0) * f(x1) > 0.0 {
        x0 = x1;
        x1 = x0 + delta;
    }
    Some((x0 + x1) / 2.0)
}

pub fn bisection(f: fn(f32) -> f32, start: f32, end: f32, delta: f32) -> Option<f32> {
    if start > end {
        return None;
    }
    if f(start) * f(end) > 0.0 {
        return None;
    }
    let mut x0 = start;
    let mut xn = end;
    let mut xc: f32;
    let mut f0fc: f32;
    loop {
        xc = (xn + x0) / 2.0;
        f0fc = f(x0) * f(xc);
        if f0fc > 0.0 {
            x0 = xc;
        } else {
            xn = xc;
        }
        if (xn - x0 < delta) || f0fc == 0.0 {
            return Some(xc);
        }
    }
}
