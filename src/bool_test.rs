use crate::sets::AtomOrSeq;
use crate::sets::AtomOrSeq::Atom;
use crate::sets::AtomOrSeq::Seq;
use crate::sets::FALSE;
use crate::sets::TRUE;
use crate::sets::PHI;

use crate::bool_fns::and;
use crate::bool_fns::or;
use crate::bool_fns::eq;

#[test]
fn and_test() {
    let v_1 = vec![TRUE.clone() , TRUE.clone()];
    let v_2 = vec![TRUE.clone() , FALSE.clone()];
    let v_3 = vec![FALSE.clone(), TRUE.clone()];
    let v_4 = vec![FALSE.clone(), FALSE.clone()];
    let v_5 = vec![FALSE.clone(), FALSE.clone(), FALSE.clone()];

    assert!(and(Some(Seq(v_1))).unwrap().bool_value().unwrap() == true);
    assert!(and(Some(Seq(v_2))).unwrap().bool_value().unwrap() == false);
    assert!(and(Some(Seq(v_3))).unwrap().bool_value().unwrap() == false);
    assert!(and(Some(Seq(v_4))).unwrap().bool_value().unwrap() == false);

    assert!(and(Some(Seq(v_5))).is_none());
    assert!(and(Some(PHI.clone())).is_none());
    assert!(and(Some(FALSE.clone())).is_none());
    assert!(and(Some(TRUE.clone())).is_none());
}

#[test]
fn or_test() {
    let v_1 = vec![TRUE.clone() , TRUE.clone()];
    let v_2 = vec![TRUE.clone() , FALSE.clone()];
    let v_3 = vec![FALSE.clone(), TRUE.clone()];
    let v_4 = vec![FALSE.clone(), FALSE.clone()];
    let v_5 = vec![FALSE.clone(), FALSE.clone(), FALSE.clone()];

    assert!(or(Some(Seq(v_1))).unwrap().bool_value().unwrap() == true);
    assert!(or(Some(Seq(v_2))).unwrap().bool_value().unwrap() == true);
    assert!(or(Some(Seq(v_3))).unwrap().bool_value().unwrap() == true);
    assert!(or(Some(Seq(v_4))).unwrap().bool_value().unwrap() == false);

    assert!(or(Some(Seq(v_5))).is_none());
    assert!(or(Some(PHI.clone())).is_none());
    assert!(or(Some(FALSE.clone())).is_none());
    assert!(or(Some(TRUE.clone())).is_none());
}

#[test]
fn eq_test() {
    let v_1 = vec![TRUE.clone() , TRUE.clone()];
    let v_2 = vec![TRUE.clone() , FALSE.clone()];
    let v_3 = vec![FALSE.clone(), TRUE.clone()];
    let v_4 = vec![FALSE.clone(), FALSE.clone()];
    let v_5 = vec![FALSE.clone(), FALSE.clone(), FALSE.clone()];

    let v_6 = vec![Seq(v_1.clone()), Seq(v_1.clone())];
    let v_7 = vec![Seq(v_2.clone()), Seq(v_2.clone())];
    let v_8 = vec![Seq(v_3.clone()), Seq(v_3.clone())];
    let v_9 = vec![Seq(v_4.clone()), Seq(v_4.clone())];

    let v_10 = vec![Seq(v_1.clone()), Seq(v_2.clone())];
    let v_11 = vec![Seq(v_1.clone()), Seq(v_3.clone())];
    let v_12 = vec![Seq(v_1.clone()), Seq(v_4.clone())];

    assert!(eq(Some(Seq(v_1))).unwrap().bool_value().unwrap() == true);
    assert!(eq(Some(Seq(v_2))).unwrap().bool_value().unwrap() == false);
    assert!(eq(Some(Seq(v_3))).unwrap().bool_value().unwrap() == false);
    assert!(eq(Some(Seq(v_4))).unwrap().bool_value().unwrap() == true);

    assert!(eq(Some(Seq(v_6))).unwrap().bool_value().unwrap() == true);
    assert!(eq(Some(Seq(v_7))).unwrap().bool_value().unwrap() == true);
    assert!(eq(Some(Seq(v_8))).unwrap().bool_value().unwrap() == true);
    assert!(eq(Some(Seq(v_9))).unwrap().bool_value().unwrap() == true);

    assert!(eq(Some(Seq(v_10))).unwrap().bool_value().unwrap() == false);
    assert!(eq(Some(Seq(v_11))).unwrap().bool_value().unwrap() == false);
    assert!(eq(Some(Seq(v_12))).unwrap().bool_value().unwrap() == false);

    assert!(eq(Some(Seq(v_5))).is_none());
    assert!(eq(Some(PHI.clone())).is_none());
    assert!(eq(Some(FALSE.clone())).is_none());
    assert!(eq(Some(TRUE.clone())).is_none());

    assert!(eq(None).is_none());
}


#[test]
fn b2() {
    assert!(TRUE.bool_value().unwrap() == true);
    assert!(FALSE.bool_value().unwrap() == false);
    assert!(PHI.bool_value().is_none());
}
