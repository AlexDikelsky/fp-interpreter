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


// pub fn construct(
//     fs: Vec<Option<&'static dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>>>) 
//         -> Box<dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>> {
//     Box::new(move |x| {
//         let fxs: Vec<Option<AtomOrSeq>> = fs.iter().map(|f| match f {
//             Some(f) => f(x.clone()),
//             _ => None,
//         }).collect();
// 
//         dbg!(&fxs);
// 
//         if fxs.iter().all(|x| x.is_some()) {
//             Some(PHI.clone())
//         } else {
//             None
//         }
//     })
//         
// }

// pub fn construct_2(f: Box<dyn Fn(usize) -> usize>, g: Box<dyn Fn(usize) -> usize>) -> Box<dyn Fn(usize) -> Vec<usize>>
//     Box::new(move |x| {
//         let v = vec![f(x.clone()), g(x.clone())];
//         if v.iter().all(|x| x.is_some()) {
//             Some(Seq(v.into_iter().map(|x| x.unwrap()).collect()))
//         } else {
//             None
//         }
//     })
// }
