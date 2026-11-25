use super::interval::{I, hull};
use super::{Arm, Datum, Geometry, MAX, Refusal, Result, Stock};
use std::array::from_fn;

pub fn arm(source: &Datum<'_>) -> Result<Arm> {
    let n = source.first.len();
    let row = |i| source.end.index(i, n);
    let [x, y, z, _] = source.first[row(0)];
    let head = [x, y, z];
    let rim = source.last[row(0)];
    let gap = from_fn(|i| I::p(rim[i]).sub(I::p(head[i])));
    let mut wd = [I::Z; MAX];
    let mut hom = [[I::Z; MAX]; 6];
    for i in 0..n {
        let a = source.first[row(i)];
        let b = source.last[row(i)];
        let w = a[3];
        wd[i] = I::p(w);
        for k in 0..3 {
            let ap = I::p(a[k]).sub(I::p(head[k]));
            let bp = I::p(b[k]).sub(I::p(rim[k]));
            hom[k][i] = ap.mul(wd[i]);
            hom[k + 3][i] = bp.sub(ap).mul(wd[i]);
        }
    }
    let pre = source.cut;
    let w = prefix(wd, n, pre);
    let rows = hom.map(|r| prefix(r, n, pre));
    let mut rate = [I::Z; 6];
    let mut speed = [I::Z; 6];
    let mut kappa = [I::Z; 6];
    for i in 0..6 {
        let (tangent, bound, curve) = derivatives(&rows[i][..n], &w[..n])?;
        rate[i] = tangent;
        speed[i] = bound;
        kappa[i] = curve;
    }
    let transform = |v: [I; 3], shift: bool| transform(source.location, v, shift);
    let v = transform(rate[..3].try_into().map_err(|_| Refusal::Source)?, false);
    let g = transform(gap, false);
    let origin = transform(head.map(I::p), true);
    let mass = |p: [I; 3]| p.into_iter().fold(I::Z, |sum, i| sum.add(I::p(i.abs()))).hi;
    Ok(Arm {
        native: Geometry {
            origin,
            v,
            k: mass(transform(
                kappa[..3].try_into().map_err(|_| Refusal::Source)?,
                false,
            )),
        },
        g,
        gp: mass(transform(
            speed[3..].try_into().map_err(|_| Refusal::Source)?,
            false,
        )),
        gpp: mass(transform(
            kappa[3..].try_into().map_err(|_| Refusal::Source)?,
            false,
        )),
    })
}

fn transform(loc: [f64; 13], world: [I; 3], shift: bool) -> [I; 3] {
    from_fn(|i| {
        let origin = if shift { I::p(loc[1 + i]) } else { I::Z };
        (0..3).fold(origin, |p, j| p.add(I::p(loc[4 + 3 * i + j]).mul(world[j])))
    })
}

fn prefix(mut row: [I; MAX], n: usize, t: I) -> [I; MAX] {
    let mut result = [I::Z; MAX];
    result[0] = row[0];
    let left = I::p(1.).sub(t);
    for (k, coefficient) in result.iter_mut().enumerate().take(n).skip(1) {
        for j in 0..n - k {
            row[j] = row[j].mul(left).add(row[j + 1].mul(t));
        }
        *coefficient = row[0];
    }
    result
}

pub(super) fn derivative(src: &[I]) -> ([I; MAX], usize) {
    let mut out = [I::Z; MAX];
    let n = src.len() - 1;
    for j in 0..n {
        out[j] = src[j + 1].sub(src[j]).mul(I::p(n as f64));
    }
    (out, n.max(1))
}

fn derivatives(column: &[I], weight: &[I]) -> Result<(I, I, I)> {
    let inverse = hull(weight).inv().ok_or(Refusal::Source)?;
    let inverse2 = inverse.sq();
    let inverse3 = inverse.mul(inverse2);
    let (wp, wn) = derivative(weight);
    let (wpp, wnn) = derivative(&wp[..wn]);
    let (hp, hn) = derivative(column);
    let (hpp, hnn) = derivative(&hp[..hn]);
    let center = hull(column);
    let hp_hull = hull(&hp[..hn]);
    let wp_hull = hull(&wp[..wn]);
    let speed = hp_hull.mul(inverse).sub(center.mul(wp_hull).mul(inverse2));
    let aa = hull(&hpp[..hnn]).mul(inverse);
    let bb = I::p(2.).mul(hp_hull).mul(wp_hull).mul(inverse2);
    let cc = center.mul(hull(&wpp[..wnn])).mul(inverse2);
    let dd = I::p(2.).mul(center).mul(wp_hull.sq()).mul(inverse3);
    let curvature = aa.sub(bb).sub(cc).add(dd);
    let tangent = hp[0].div(weight[0]).ok_or(Refusal::Source)?;
    Ok((tangent, speed, curvature))
}

pub fn cone_image(stock: &Stock, datum: &Datum<'_>, width: f64) -> Result<Geometry> {
    if !(0. ..=1.).contains(&width) || width == 0. {
        return Err(Refusal::Source);
    }
    let [u, v] = datum.pcurve[0];
    let [cu, su] = super::interval::smooth_trig(u).ok_or(Refusal::Source)?;
    let [ca, sa] = stock.trig;
    let mut derivative_bounds = [[I::Z; 3]; 2];
    let mut curvature_bounds = [[I::Z; 2]; 2];
    for i in 0..2 {
        let mut raw = [I::Z; MAX];
        for (j, p) in datum.pcurve.iter().enumerate() {
            raw[j] = I::p(p[i]).sub(I::p(datum.pcurve[0][i]));
        }
        let clipped = prefix(raw, 4, I::p(width));
        let (d, n) = derivative(&clipped[..4]);
        let (dd, m) = derivative(&d[..n]);
        derivative_bounds[i].copy_from_slice(&d[..n]);
        curvature_bounds[i].copy_from_slice(&dd[..m]);
    }
    let [up, vp] = derivative_bounds;
    let vp0 = vp[0];
    let up0 = up[0];
    let radius = I::p(stock.radius).add(sa.mul(I::p(v)));
    let radial = sa.mul(vp0);
    let angular = radius.mul(up0);
    let origin = [
        radius.mul(cu),
        radius.mul(su),
        I::p(stock.z).add(ca.mul(I::p(v))),
    ];
    let v = [
        cu.mul(radial).sub(su.mul(angular)),
        su.mul(radial).add(cu.mul(angular)),
        ca.mul(vp0),
    ];
    let radius_max = I::p(stock.radius).add(
        sa.mul(I::p(
            datum
                .pcurve
                .iter()
                .map(|p| p[1])
                .fold(f64::NEG_INFINITY, f64::max),
        )),
    );
    let umax = I::p(hull(&up).abs());
    let vmax = I::p(hull(&vp).abs());
    let u2 = I::p(hull(&curvature_bounds[0]).abs());
    let v2 = I::p(hull(&curvature_bounds[1]).abs());
    let k = v2
        .add(I::p(2.).mul(sa).mul(umax).mul(vmax))
        .add(radius_max.mul(umax.sq().add(u2)))
        .hi;
    Ok(Geometry { origin, v, k })
}
