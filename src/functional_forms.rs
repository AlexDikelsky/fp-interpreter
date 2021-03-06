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
use crate::sets::PHI;

#[allow(dead_code)]
pub fn compose(f: Box<FunctionalForm>, g: Box<FunctionalForm>) -> Box<FunctionalForm> {
    Box::new(move |x| f(g(x)))
}

#[allow(dead_code)]
pub fn construct_usize_2(f: Box<dyn Fn(usize) -> usize>, g: Box<dyn Fn(usize) -> usize>) -> Box<dyn Fn(usize) -> Vec<usize>> {
    Box::new(move |x| {
        vec![f(x), g(x)]
    })
}

#[allow(dead_code)]
pub fn construct_usize(fs: Vec<Box<dyn Fn(usize) -> usize>>) -> Box<dyn Fn(usize) -> Vec<usize>> {
    Box::new(move |x| {
        fs.iter().map(|f| f(x)).collect()
    })
}


#[allow(dead_code)]
pub fn construct_bad(
    fs: Vec<Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>>>) 
        -> Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>> {
    Box::new(move |x| {
        Some(Seq(fs.iter().map(|f| f(x.clone()).unwrap()).collect()))
    })
}

#[allow(dead_code)]
pub fn construct_unsafe(
    fs: Vec<Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>>>) 
        -> Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>> {

    Box::new(move |x| {
        let v: Vec<Option<AtomOrSeq>> = fs.iter().map(|f| f(x.clone())).collect();
        if v.iter().all(|x| x.is_some()) {
            Some(Seq(v.into_iter().map(|x| x.unwrap()).collect()))
        } else {
            None
        }
    })
}

#[allow(dead_code)]
pub fn construct(
    fs: Vec<Option<Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>>>>) 
        -> Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>> {
    if fs.iter().all(|f| f.is_some()) {
        let fs: Vec<Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>>> 
            = fs.into_iter().map(|x| x.unwrap()).collect();
        Box::new(move |x| {
            let v: Vec<Option<AtomOrSeq>> = fs.iter().map(|f| f(x.clone())).collect();
            if v.iter().all(|x| x.is_some()) {
                Some(Seq(v.into_iter().map(|x| x.unwrap()).collect()))
            } else {
                None
            }
        })
    } else {
        Box::new(|x| None)
    }
}

