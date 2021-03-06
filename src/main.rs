#[macro_use]
extern crate lazy_static;

mod sets;
mod bool_test;
mod bool_fns;

use crate::sets::AtomOrSeq;
use crate::sets::AtomOrSeq::Atom;
use crate::sets::AtomOrSeq::Seq;
use crate::sets::FALSE;
use crate::sets::TRUE;
use crate::sets::from_bool;


fn main() {
    println!("Hello, world!");
    dbg!(eq_fp("abc", "abc"));
}

fn eq_fp<T: PartialEq>(x: T, y: T) -> &'static AtomOrSeq<'static> {
    if x == y {
        &TRUE
    } else {
        &FALSE
    }
}



// fn eq(x: Option<AtomOrSeq>) -> Option<AtomOrSeq> {
//     match x {
//         None => None,
//         Some(x) => match x {
//             Atom(_) => None,
//             Seq(x) => match (x.get(0), x.get(1)) {
//                 (None, _) => None,
//                 (_, None) => None,
//                 (Some(a), Some(b)) => match (a, b) {
//                     (Atom(a), Atom(b)) => Some(eq_fp(a, b)),
//                     (Seq(a), Seq(b)) => {
//                         zip_eq(a.iter(), b.iter()).fold(TRUE, |acc, (a_i, b_i)| {
