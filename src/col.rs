use super::math::*;
use super::util::*;

#[derive(PartialEq)]
enum Ori {
    Collinear,
    Clock,
    Anticlock,
}

pub fn point_in_polygon(p1: &APoint, poly: &Vec<APoint>) {}

pub fn lines_intersect(l1: (APoint, APoint), l2: (APoint, APoint)) -> bool {
    let o1 = orientation(l1.0, l1.1, l2.0);
    let o2 = orientation(l1.0, l1.1, l2.1);
    let o3 = orientation(l2.0, l2.1, l1.0);
    let o4 = orientation(l2.0, l2.1, l1.1);
    if o1 != o2 && o3 != o4 {
        return true;
    }
    if o1 == Ori::Collinear && on_segment(l1.0, l2.0, l1.1) {
        return true;
    }
    // p1, q1 and q2 are colinear and q2 lies on segment p1q1
    if o2 == Ori::Collinear && on_segment(l1.0, l2.1, l1.1) {
        return true;
    };

    // p2, l2[1] and l1[0] are colinear and l1[0] lies on segment p2l2[1]
    if o3 == Ori::Collinear && on_segment(l2.0, l1.0, l2.1) {
        return true;
    };

    // l2.0, l2[1] and l1[1] are colinear and l1[1] lies on segment l2.0l2[1]
    if o4 == Ori::Collinear && on_segment(l2.0, l1.1, l2.1) {
        return true;
    };
    false
}

fn on_segment(p: APoint, q: APoint, r: APoint) -> bool {
    if q[0] <= p[0].max(r[0])
        && q[0] >= p[0].min(r[0])
        && q[1] <= p[1].max(r[1])
        && q[1] >= p[1].min(r[1])
    {
        return true;
    }
    false
}

fn orientation(p: APoint, q: APoint, r: APoint) -> Ori {
    let val = (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1]);

    return if -0.01 <= val && val <= 0.01 {
        Ori::Collinear
    } else if val > 0.01 {
        Ori::Clock
    } else {
        Ori::Anticlock
    };
}
