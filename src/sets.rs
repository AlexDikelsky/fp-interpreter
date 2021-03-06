use crate::sets::AtomOrSeq::Atom;
use crate::sets::AtomOrSeq::Seq;

pub type FunctionalForm = dyn Fn(Option<AtomOrSeq>) -> Option<AtomOrSeq>;

#[derive(Debug)]
pub enum Set<'a> {
    Object(Option<AtomOrSeq<'a>>),
    Function,
    FunctionalForm,
    Definition
}

#[derive(Debug, Clone)]
pub enum AtomOrSeq<'a> {
    Atom(&'a str),
    Seq(Vec<AtomOrSeq<'a>>)
}

pub const TRUE_STR: &str = &"T";
pub const FALSE_STR: &str = &"F";

lazy_static! {
    pub static ref TRUE: AtomOrSeq<'static> = Atom("T");
    pub static ref FALSE: AtomOrSeq<'static> = Atom("F");
    pub static ref PHI: AtomOrSeq<'static> = Seq(vec![]);
}

pub fn from_bool(x: bool) -> AtomOrSeq<'static> {
    if x {
        TRUE.clone()
    } else {
        FALSE.clone()
    }
}

impl<'a> AtomOrSeq<'a> {
    pub fn bool_value(&'a self) -> Option<bool> {
        match &self {
            Atom(x) => 
                if *x == TRUE_STR {
                    Some(true)
                } else if *x == FALSE_STR {
                    Some(false)
                } else {
                    None
                }
            _ => None
        }
    }
}
