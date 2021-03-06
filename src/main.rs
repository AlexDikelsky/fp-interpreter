#[macro_use]
extern crate lazy_static;

mod sets;
mod bool_test;
mod bool_fns;
mod functional_forms;
mod functions;

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
use crate::sets::FunctionalForm;

#[allow(unused_imports)]
use crate::functions::id;


#[allow(unused_imports)]
use crate::functional_forms::construct_usize;
#[allow(unused_imports)]
use crate::functional_forms::construct;
#[allow(unused_imports)]
use crate::functional_forms::construct_unsafe;
#[allow(unused_imports)]
use crate::bool_fns::and;
#[allow(unused_imports)]
use crate::bool_fns::eq;


fn p(x: usize) -> usize {
    x + 1
}

fn main() {
    println!("Hello, world!");

    let fs: Vec<Box<Fn(usize) -> usize>> = vec![Box::new(p), Box::new(|x| x + 2)];
    dbg!(&construct_usize(fs)(3));


    let gs: Vec<Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>>> =
        vec![Box::new(id), Box::new(id), Box::new(and)];

    let h = Some(TRUE.clone());

    dbg!(construct_unsafe(gs)(h.clone()));


    // let f = construct_usize_2(Box::new(|x| x + 1), Box::new(|x| x + 2));
    // dbg!(&f(3));

    // let f = construct_2(Box::new(eq), Box::new(eq));

    // let v: AtomOrSeq = Seq(vec![TRUE.clone(), TRUE.clone()]);

    // let f: Box<Box<dyn for<'r> Fn(Option<AtomOrSeq<'r>>) -> Option<AtomOrSeq<'r>>>>
    //     = Box::new(construct(vec![Some(&eq)]));
}
