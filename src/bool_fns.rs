#[allow(unused_imports)]
use crate::sets::AtomOrSeq;
#[allow(unused_imports)]
use crate::sets::AtomOrSeq::Atom;
#[allow(unused_imports)]
use crate::sets::AtomOrSeq::Seq;
#[allow(unused_imports)]
use crate::sets::FALSE;
#[allow(unused_imports)]
use crate::sets::TRUE;
#[allow(unused_imports)]
use crate::sets::from_bool;
#[allow(unused_imports)]
use itertools::zip_eq;


#[allow(dead_code)]
pub fn and<'a>(x: Option<AtomOrSeq>) -> Option<&'a AtomOrSeq> {
    bool_fn(x, Box::new(|x, y| x && y))
}

#[allow(dead_code)]
pub fn or<'a>(x: Option<AtomOrSeq>) -> Option<&'a AtomOrSeq> {
    bool_fn(x, Box::new(|x, y| x || y))
}

#[allow(dead_code)]
fn bool_fn<'a>(x: Option<AtomOrSeq>, f: Box<Fn(bool, bool) -> bool>) -> Option<&'a AtomOrSeq> {
    match x {
        Some(x) => match x {
            Seq(s) => match (s.get(0), s.get(1), s.get(2)) {
                (Some(x), Some(y), None) => match (x.bool_value(), y.bool_value()) {
                    (Some(x), Some(y)) => Some(from_bool(f(x, y))),
                    _ => None,
                },
                _ => None,
            }
            Atom(_) => None,
        }
        None => None,
    }
}

// Should be using a find rather than a fold here so that it short circuts
#[allow(dead_code)]
pub fn eq<'a>(x: Option<AtomOrSeq>) -> Option<&'a AtomOrSeq> {
    match x {
        Some(x) => match x {
            Seq(s) => match (s.get(0), s.get(1), s.get(2)) {
                (Some(y), Some(z), None) => match (y, z) {
                    (Seq(ys), Seq(zs)) => {
                        zip_eq(ys.iter(), zs.iter())
                            .fold(Some(&*TRUE), |acc, (y_i, z_i)| {
                                match (eq(Some(Seq(vec![y_i.clone(), z_i.clone()]))), acc) {
                                    (Some(k), Some(acc)) => match k.bool_value() {
                                        Some(a) => match a {
                                            true => and(Some(Seq(vec![acc.clone(), TRUE.clone()]))),
                                            false => Some(&*FALSE),
                                        }
                                        None => None
                                    },
                                    _ => None
                                }
                            })
                    },
                    (Atom(a), Atom(b)) => Some(from_bool(a == b)),
                    _ => None,
                },
                _ => None
            },
            _ => None
        },
        _ => None
    }
}
