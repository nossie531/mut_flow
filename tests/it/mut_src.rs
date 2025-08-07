use crate::for_test::MyType;
use mut_flow::prelude::*;
use std::ptr;
use test_panic::prelude::*;

#[test]
fn new() {
    let mut_ref = &mut 42;
    let result = &mut MutSrc::new(mut_ref);
    assert!(result.is_enabled());
    assert_eq!(result.value(), &42);
    assert_eq!(result.value_mut(), &42);
}

#[test]
fn alt() {
    let mut_ref = &mut MyType::new();
    let target = &mut MutSrc::new(mut_ref);
    let result = target.alt::<i32>();
    assert!(!result.is_enabled());
    assert!(result.is_from(target));
}

#[test]
fn is_enabled() {
    case_false();
    case_true();

    fn case_false() {
        let mut_ref = &mut MyType::new();
        let target = &mut MutSrc::new(mut_ref);
        let alt = &mut target.alt::<i32>();
        alt.switch(target, |x| x.fld2);
        let result = target.is_enabled();
        assert!(!result);
    }

    fn case_true() {
        let mut_ref = &mut MyType::new();
        let target = &mut MutSrc::new(mut_ref);
        let result = target.is_enabled();
        assert!(result);
    }
}

#[test]
fn value() {
    normal();
    at_disabled();

    fn normal() {
        let mut_ref = &mut 42;
        let target = &mut MutSrc::new(mut_ref);
        let result = target.value();
        assert!(ptr::eq(result, mut_ref));
    }

    fn at_disabled() {
        let mut_ref = &mut MyType::new();
        let target = &mut MutSrc::new(mut_ref);
        let alt = &mut target.alt::<i32>();
        alt.switch(target, |x| x.fld2);
        let result = test_panic(|| target.value());
        assert!(result.is_panic());
    }
}

#[test]
fn value_mut() {
    normal();
    at_disabled();

    fn normal() {
        let mut_ref = &mut 42;
        let target = &mut MutSrc::new(mut_ref);
        let result = target.value_mut();
        assert!(ptr::eq(result, mut_ref));
    }

    fn at_disabled() {
        let mut_ref = &mut MyType::new();
        let target = &mut MutSrc::new(mut_ref);
        let alt = &mut target.alt::<i32>();
        alt.switch(target, |x| x.fld2);
        let result = test_panic(|| target.value_mut());
        assert!(result.is_panic());
    }
}

#[test]
fn switch() {
    normal();
    alt_is_disabled();
    different_src();

    fn normal() {
        let mut_ref = &mut MyType::new().update_fld1(1);
        let target = &mut MutSrc::new(mut_ref);
        let alt = &mut target.alt::<i32>();
        alt.switch(target, |x| x.fld2);
        target.switch(alt);

        assert!(target.is_enabled());
        assert_eq!(target.value().fld1, 1);
        assert_eq!(target.value_mut().fld1, 1);
        assert!(!alt.is_enabled());
        assert!(alt.is_from(target));
    }

    fn alt_is_disabled() {
        let mut_ref = &mut MyType::new();
        let target = &mut MutSrc::new(mut_ref);
        let alt = &mut target.alt::<i32>();
        let result = test_panic(|| target.switch(alt));
        assert!(result.is_panic());
    }

    fn different_src() {
        let mut_ref1 = &mut MyType::new();
        let mut_ref2 = &mut MyType::new();
        let target1 = &mut MutSrc::new(mut_ref1);
        let target2 = &mut MutSrc::new(mut_ref2);
        let alt2 = &mut target2.alt::<i32>();
        let result = test_panic(|| alt2.switch(target1, |x| x.fld2));
        assert!(result.is_panic());
    }
}
