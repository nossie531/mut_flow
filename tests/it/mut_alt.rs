use crate::for_test::MyType;
use mut_flow::prelude::*;
use test_panic::prelude::*;

#[test]
fn is_enabled() {
    case_false();
    case_true();

    fn case_false() {
        let mut_ref = &mut MyType::new();
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        assert!(!target.is_enabled());
    }

    fn case_true() {
        let mut_ref = &mut MyType::new();
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        target.switch(src, |x| x.fld2);
        assert!(target.is_enabled());
    }
}

#[test]
fn is_same_src() {
    case_false();
    case_true();

    fn case_false() {
        let mut_ref1 = &mut MyType::new();
        let mut_ref2 = &mut MyType::new();
        let src1 = &mut MutSrc::new(mut_ref1);
        let src2 = &mut MutSrc::new(mut_ref2);
        let target = &mut src1.alt::<i32>();
        let result = target.is_from(src2);
        assert!(!result);
    }

    fn case_true() {
        let mut_ref = &mut MyType::new();
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        let result = target.is_from(src);
        assert!(result);
    }
}

#[test]
fn value() {
    normal();
    at_disabled();

    fn normal() {
        let mut_ref = &mut MyType::new().update_fld2(2);
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        target.switch(src, |x| x.fld2);
        let result = target.value();
        assert_eq!(*result, 2);
    }

    fn at_disabled() {
        let mut_ref = &mut MyType::new();
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        let result = test_panic(|| target.value());
        assert!(result.is_panic());
    }
}

#[test]
fn value_mut() {
    normal();
    at_disabled();

    fn normal() {
        let mut_ref = &mut MyType::new().update_fld2(2);
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        target.switch(src, |x| x.fld2);
        let result = target.value_mut();
        assert_eq!(*result, 2);
    }

    fn at_disabled() {
        let mut_ref = &mut MyType::new();
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        let result = test_panic(|| target.value_mut());
        assert!(result.is_panic());
    }
}

#[test]
fn switch() {
    normal();
    src_is_disabled();
    different_src();

    fn normal() {
        let mut_ref = &mut MyType::new().update_fld2(2);
        let src = &mut MutSrc::new(mut_ref);
        let target = &mut src.alt::<i32>();
        target.switch(src, |x| x.fld2);
        assert!(target.is_enabled());
        assert_eq!(*target.value(), 2);
        assert_eq!(*target.value_mut(), 2);
        assert!(!src.is_enabled());
        assert!(target.is_from(src));
    }

    fn src_is_disabled() {
        let mut_ref = &mut MyType::new();
        let src = &mut MutSrc::new(mut_ref);
        let target1 = &mut src.alt::<i32>();
        let target2 = &mut src.alt::<i32>();
        target1.switch(src, |x| x.fld2);
        let result = test_panic(|| target2.switch(src, |x| x.fld2));
        assert!(result.is_panic());
    }

    fn different_src() {
        let mut_ref1 = &mut MyType::new();
        let mut_ref2 = &mut MyType::new();
        let src1 = &mut MutSrc::new(mut_ref1);
        let src2 = &mut MutSrc::new(mut_ref2);
        let target1 = &mut src1.alt::<i32>();
        let result = test_panic(|| target1.switch(src2, |x| x.fld2));
        assert!(result.is_panic());
    }
}
