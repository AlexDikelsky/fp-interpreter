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

pub fn id(x: Option<AtomOrSeq>) -> Option<AtomOrSeq> {
    x
}
