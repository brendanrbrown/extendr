use crate::*;

#[test]
fn test_debug() {
    test! {
        // Special values
        assert_eq!(format!("{:?}", r!(NULL)), "r!(NULL)");
        assert_eq!(format!("{:?}", r!(TRUE)), "r!(TRUE)");
        assert_eq!(format!("{:?}", r!(FALSE)), "r!(FALSE)");

        // Scalars
        assert_eq!(format!("{:?}", r!(1)), "r!(1)");
        assert_eq!(format!("{:?}", r!(1.)), "r!(1.0)");
        assert_eq!(format!("{:?}", r!("hello")), "r!([\"hello\"])");
        let s = "hello".to_string();
        assert_eq!(format!("{:?}", r!(s)), "r!([\"hello\"])");

        // Vectors
        assert_eq!(format!("{:?}", r!([1, 2, 3])), "r!([1, 2, 3])");
        assert_eq!(format!("{:?}", r!([1., 2., 3.])), "r!([1.0, 2.0, 3.0])");
        assert_eq!(format!("{:?}", r!(Raw::from_bytes(&[1, 2, 3]))), "r!(Raw::from_bytes([1, 2, 3]))");

        // Wrappers
        assert_eq!(format!("{:?}", r!(Symbol::from_string("x"))), "sym!(x)");
        assert_eq!(format!("{:?}", r!(Character::from_string("x"))), "r!(Character::from_string(\"x\"))");
        assert_eq!(
            format!("{:?}", r!(Language::from_values(&[r!(Symbol::from_string("x"))]))),
            "r!(Language::from_values([sym!(x)]))"
        );

        // Logical
        assert_eq!(
            format!("{:?}", r!([TRUE, FALSE, NA_LOGICAL])),
            "r!([TRUE, FALSE, NA_LOGICAL])"
        );
    }
}

#[test]
fn test_from_robj() {
    test! {
        assert_eq!(<bool>::from_robj(&Robj::from(true)), Ok(true));
        assert_eq!(<u8>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<u16>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<u32>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<u64>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i8>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i16>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i32>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<i64>::from_robj(&Robj::from(1)), Ok(1));
        assert_eq!(<f32>::from_robj(&Robj::from(1)), Ok(1.));
        assert_eq!(<f64>::from_robj(&Robj::from(1)), Ok(1.));

        assert_eq!(<Vec::<i32>>::from_robj(&Robj::from(1)), Ok(vec![1]));
        assert_eq!(<Vec::<f64>>::from_robj(&Robj::from(1.)), Ok(vec![1.]));

        let hello = Robj::from("hello");
        assert_eq!(<&str>::from_robj(&hello), Ok("hello"));

        // conversion from a vector to a scalar value
        assert_eq!(
            <i32>::from_robj(&Robj::from(vec![].as_slice() as &[i32])),
            Err("Input must be of length 1. Vector of length zero given.")
        );
        assert_eq!(
            <i32>::from_robj(&Robj::from(vec![1].as_slice() as &[i32])),
            Ok(1)
        );
        assert_eq!(
            <i32>::from_robj(&Robj::from(vec![1, 2].as_slice() as &[i32])),
            Err("Input must be of length 1. Vector of length >1 given.")
        );

        use std::collections::HashMap;
        let list = eval_string("list(a = 1L, b = 2L)").unwrap();
        let hmap1 = [("a".into(), 1.into()), ("b".into(), 2.into())]
            .iter()
            .cloned()
            .collect::<HashMap<String, Robj>>();
        let hmap2 = [("a", 1.into()), ("b", 2.into())]
            .iter()
            .cloned()
            .collect::<HashMap<&str, Robj>>();
        let hmap_owned = <HashMap<String, Robj>>::from_robj(&list).unwrap();
        let hmap_borrowed = <HashMap<&str, Robj>>::from_robj(&list).unwrap();
        assert_eq!(hmap_owned, hmap1);
        assert_eq!(hmap_borrowed, hmap2);

        assert_eq!(hmap_owned["a"], Robj::from(1));
        assert_eq!(hmap_owned["b"], Robj::from(2));

        assert_eq!(hmap_borrowed["a"], Robj::from(1));
        assert_eq!(hmap_borrowed["b"], Robj::from(2));

        let na_integer = eval_string("NA_integer_").unwrap();
        assert!(<i32>::from_robj(&na_integer).is_err());
        assert_eq!(<Option<i32>>::from_robj(&na_integer), Ok(None));
        assert_eq!(<Option<i32>>::from_robj(&Robj::from(1)), Ok(Some(1)));
        assert!(<Option<i32>>::from_robj(&Robj::from([1, 2])).is_err());

        let na_bool = eval_string("TRUE == NA").unwrap();
        assert!(<bool>::from_robj(&na_bool).is_err());
        assert_eq!(<Option<bool>>::from_robj(&na_bool), Ok(None));
        assert_eq!(<Option<bool>>::from_robj(&Robj::from(true)), Ok(Some(true)));
        assert!(<Option<bool>>::from_robj(&Robj::from([true, false])).is_err());

        let na_real = eval_string("NA_real_").unwrap();
        assert!(<f64>::from_robj(&na_real).is_err());
        assert_eq!(<Option<f64>>::from_robj(&na_real), Ok(None));
        assert_eq!(<Option<f64>>::from_robj(&Robj::from(1.)), Ok(Some(1.)));
        assert!(<Option<f64>>::from_robj(&Robj::from([1., 2.])).is_err());

        let na_string = eval_string("NA_character_").unwrap();
        assert!(<&str>::from_robj(&na_string).is_err());
        assert_eq!(<Option<&str>>::from_robj(&na_string), Ok(None));
        assert_eq!(<Option<&str>>::from_robj(&Robj::from("1")), Ok(Some("1")));
        assert!(<Option<&str>>::from_robj(&Robj::from(["1", "2"])).is_err());

        let na_string = eval_string("NA_character_").unwrap();
        assert!(<String>::from_robj(&na_string).is_err());
        assert_eq!(<Option<String>>::from_robj(&na_string), Ok(None));
        assert_eq!(
            <Option<String>>::from_robj(&Robj::from("1")),
            Ok(Some("1".to_string()))
        );
        assert!(<Option<String>>::from_robj(&Robj::from(["1", "2"])).is_err());
    }
}

#[test]
fn test_try_from_robj() {
    test! {
        assert_eq!(<bool>::try_from(Robj::from(true)), Ok(true));
        assert_eq!(<u8>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<u16>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<u32>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<u64>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<i8>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<i16>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<i32>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<i64>::try_from(Robj::from(1)), Ok(1));
        assert_eq!(<f32>::try_from(Robj::from(1)), Ok(1.));
        assert_eq!(<f64>::try_from(Robj::from(1)), Ok(1.));

        // conversion from non-integer-ish value to integer should fail
        let robj = Robj::from(1.5);
        assert_eq!(<i32>::try_from(robj.clone()), Err(Error::ExpectedWholeNumber(robj)));
        // conversion from out-of-limit value should fail
        let robj = Robj::from(32768);
        assert_eq!(<i16>::try_from(robj.clone()), Err(Error::OutOfLimits(robj)));
        let robj = Robj::from(-1);
        assert_eq!(<u32>::try_from(robj.clone()), Err(Error::OutOfLimits(robj)));

        assert_eq!(<Vec::<i32>>::try_from(Robj::from(1)), Ok(vec![1]));
        assert_eq!(<Vec::<f64>>::try_from(Robj::from(1.)), Ok(vec![1.]));
        assert_eq!(<Vec::<Bool>>::try_from(Robj::from(TRUE)), Ok(vec![TRUE]));
        assert_eq!(<Vec::<u8>>::try_from(Robj::from(0_u8)), Ok(vec![0_u8]));

        assert_eq!(<&[i32]>::try_from(Robj::from(1)), Ok(&[1][..]));
        assert_eq!(<&[f64]>::try_from(Robj::from(1.)), Ok(&[1.][..]));
        assert_eq!(<&[Bool]>::try_from(Robj::from(TRUE)), Ok(&[TRUE][..]));
        assert_eq!(<&[u8]>::try_from(Robj::from(0_u8)), Ok(&[0_u8][..]));

        // Note the Vec<> cases use the same logic as the slices.
        assert_eq!(<&[i32]>::try_from(Robj::from(1.0)), Err(Error::ExpectedInteger(r!(1.0))));
        assert_eq!(<&[f64]>::try_from(Robj::from(1)), Err(Error::ExpectedReal(r!(1))));
        assert_eq!(<&[Bool]>::try_from(Robj::from(())), Err(Error::ExpectedLogical(r!(()))));
        assert_eq!(<&[u8]>::try_from(Robj::from(())), Err(Error::ExpectedRaw(r!(()))));

        let hello = Robj::from("hello");
        assert_eq!(<&str>::try_from(hello), Ok("hello"));

        let hello = Robj::from("hello");
        assert_eq!(<String>::try_from(hello), Ok("hello".into()));

        // conversion from a vector to a scalar value
        let robj = Robj::from(vec![].as_slice() as &[i32]);
        assert_eq!(
            <i32>::try_from(robj.clone()),
            Err(Error::ExpectedNonZeroLength(robj))
        );
        assert_eq!(
            <i32>::try_from(Robj::from(vec![1].as_slice() as &[i32])),
            Ok(1)
        );
        let robj = Robj::from(vec![1, 2].as_slice() as &[i32]);
        assert_eq!(
            <i32>::try_from(robj.clone()),
            Err(Error::ExpectedScalar(robj))
        );

        use std::collections::HashMap;
        let list = eval_string("list(a = 1L, b = 2L)").unwrap();
        let hmap1 = [("a".into(), 1.into()), ("b".into(), 2.into())]
            .iter()
            .cloned()
            .collect::<HashMap<String, Robj>>();
        let hmap2 = [("a", 1.into()), ("b", 2.into())]
            .iter()
            .cloned()
            .collect::<HashMap<&str, Robj>>();
        let hmap_owned = <HashMap<String, Robj>>::try_from(list.clone()).unwrap();
        let hmap_borrowed = <HashMap<&str, Robj>>::try_from(list.clone()).unwrap();
        assert_eq!(hmap_owned, hmap1);
        assert_eq!(hmap_borrowed, hmap2);

        assert_eq!(hmap_owned["a"], Robj::from(1));
        assert_eq!(hmap_owned["b"], Robj::from(2));

        assert_eq!(hmap_borrowed["a"], Robj::from(1));
        assert_eq!(hmap_borrowed["b"], Robj::from(2));

        let na_integer = eval_string("NA_integer_").unwrap();
        assert!(<i32>::try_from(na_integer.clone()).is_err());
        assert_eq!(<Option<i32>>::try_from(na_integer.clone()), Ok(None));
        assert_eq!(<Option<i32>>::try_from(Robj::from(1)), Ok(Some(1)));
        assert!(<Option<i32>>::try_from(Robj::from([1, 2])).is_err());

        let na_bool = eval_string("TRUE == NA").unwrap();
        assert!(<bool>::try_from(na_bool.clone()).is_err());
        assert_eq!(<Option<bool>>::try_from(na_bool.clone()), Ok(None));
        assert_eq!(<Option<bool>>::try_from(Robj::from(true)), Ok(Some(true)));
        assert!(<Option<bool>>::try_from(Robj::from([true, false])).is_err());

        let na_real = eval_string("NA_real_").unwrap();
        assert!(<f64>::try_from(na_real.clone()).is_err());
        assert_eq!(<Option<f64>>::try_from(na_real.clone()), Ok(None));
        assert_eq!(<Option<f64>>::try_from(Robj::from(1.)), Ok(Some(1.)));
        assert!(<Option<f64>>::try_from(Robj::from([1., 2.])).is_err());

        let na_string = eval_string("NA_character_").unwrap();
        assert!(<&str>::try_from(na_string.clone()).is_err());
        assert_eq!(<Option<&str>>::try_from(na_string.clone()), Ok(None));
        assert_eq!(<Option<&str>>::try_from(Robj::from("1")), Ok(Some("1")));
        assert!(<Option<&str>>::try_from(Robj::from(["1", "2"])).is_err());

        let na_string = eval_string("NA_character_").unwrap();
        assert!(<String>::try_from(na_string.clone()).is_err());
        assert_eq!(<Option<String>>::try_from(na_string.clone()), Ok(None));
        assert_eq!(
            <Option<String>>::try_from(Robj::from("1")),
            Ok(Some("1".to_string()))
        );
        assert!(<Option<String>>::try_from(Robj::from(["1", "2"])).is_err());
    }
}
#[test]
fn test_to_robj() {
    test! {
        assert_eq!(Robj::from(true), Robj::from([Bool::from(true)]));
        //assert_eq!(Robj::from(1_u8), Robj::from(1));
        assert_eq!(Robj::from(1_u16), Robj::from(1));
        assert_eq!(Robj::from(1_u32), Robj::from(1.));
        assert_eq!(Robj::from(1_u64), Robj::from(1.));
        assert_eq!(Robj::from(1_usize), Robj::from(1.));
        assert_eq!(Robj::from(1_i8), Robj::from(1));
        assert_eq!(Robj::from(1_i16), Robj::from(1));
        assert_eq!(Robj::from(1_i32), Robj::from(1));
        assert_eq!(Robj::from(1_i64), Robj::from(1.));
        assert_eq!(Robj::from(1.0_f32), Robj::from(1.));
        assert_eq!(Robj::from(1.0_f64), Robj::from(1.));

        // check large values
        assert_eq!(Robj::from(i64::MAX), Robj::from(i64::MAX as f64));
        assert_eq!(Robj::from(i64::MIN), Robj::from(i64::MIN as f64));

        // check NaN and Inf
        assert_eq!(Robj::from(f64::NAN), R!("NaN")?);
        assert_eq!(Robj::from(f64::INFINITY), R!("Inf")?);
        assert_eq!(Robj::from(-f64::INFINITY), R!("-Inf")?);

        let ab = Robj::from(vec!["a", "b"]);
        let ab2 = Robj::from(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(ab, ab2);
        assert_eq!(format!("{:?}", ab), "r!([\"a\", \"b\"])");
        assert_eq!(format!("{:?}", ab2), "r!([\"a\", \"b\"])");

        assert_eq!(Robj::from(Some(1)), Robj::from(1));
        assert!(!Robj::from(Some(1)).is_na());
        assert!(Robj::from(<Option<i32>>::None).is_na());

        assert_eq!(Robj::from(Some(true)), Robj::from(true));
        assert!(!Robj::from(Some(true)).is_na());
        assert!(Robj::from(<Option<bool>>::None).is_na());

        assert_eq!(Robj::from(Some(1.)), Robj::from(1.));
        assert!(!Robj::from(Some(1.)).is_na());
        assert!(Robj::from(<Option<f64>>::None).is_na());

        assert_eq!(Robj::from(Some("xyz")), Robj::from("xyz"));
        assert!(!Robj::from(Some("xyz")).is_na());
        assert!(Robj::from(<Option<&str>>::None).is_na());
    }
}

#[test]
fn parse_test() {
    test! {
    let p = parse("print(1L);print(1L);")?;
    let q = r!(Expression::from_values(&[
        r!(Language::from_values(&[r!(Symbol::from_string("print")), r!(1)])),
        r!(Language::from_values(&[r!(Symbol::from_string("print")), r!(1)]))
    ]));
    assert_eq!(p, q);

    let p = eval_string("1L + 1L")?;
    assert_eq!(p, Robj::from(2));
    }
}

#[test]
fn output_iterator_test() {
    test! {
        // Allocation where size is known in advance.
        let robj = (0..3).collect_robj();
        assert_eq!(robj.as_integer_vector().unwrap(), vec![0, 1, 2]);

        let robj = [0, 1, 2].iter().collect_robj();
        assert_eq!(robj.as_integer_vector().unwrap(), vec![0, 1, 2]);

        let robj = (0..3).map(|x| x % 2 == 0).collect_robj();
        assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE, TRUE]);

        let robj = [TRUE, FALSE, TRUE].iter().collect_robj();
        assert_eq!(robj.as_logical_vector().unwrap(), vec![TRUE, FALSE, TRUE]);

        let robj = (0..3).map(|x| x as f64).collect_robj();
        assert_eq!(robj.as_real_vector().unwrap(), vec![0., 1., 2.]);

        let robj = [0., 1., 2.].iter().collect_robj();
        assert_eq!(robj.as_real_vector().unwrap(), vec![0., 1., 2.]);

        let robj = (0..3).map(|x| format!("{}", x)).collect_robj();
        assert_eq!(robj.as_str_vector(), Some(vec!["0", "1", "2"]));

        let robj = ["0", "1", "2"].iter().collect_robj();
        assert_eq!(robj.as_str_vector(), Some(vec!["0", "1", "2"]));

        // Fallback allocation where size is not known in advance.
        let robj = (0..3).filter(|&x| x != 1).collect_robj();
        assert_eq!(robj.as_integer_vector().unwrap(), vec![0, 2]);

        let robj = (0..3).filter(|&x| x != 1).map(|x| x as f64).collect_robj();
        assert_eq!(robj.as_real_vector().unwrap(), vec![0., 2.]);

        let robj = (0..3)
            .filter(|&x| x != 1)
            .map(|x| format!("{}", x))
            .collect_robj();
        assert_eq!(robj.as_str_vector(), Some(vec!["0", "2"]));
    }
}

// Test that we can use Iterators as the input to functions.
// eg.
// #[extendr]
// fn fred(a: Real, b: Real) -> Robj {
// }
#[test]
fn input_iterator_test() {
    test! {
        let src: &[&str] = &["1", "2", "3"];
        let robj = Robj::from(src);
        let iter = <StrIter>::from_robj(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        let src = &[Robj::from(1), Robj::from(2), Robj::from(3)];
        let robj = Robj::from(List::from_values(src));
        let iter = <ListIter>::from_robj(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        let src: &[i32] = &[1, 2, 3];
        let robj = Robj::from(src);
        let iter = <Int>::from_robj(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        let src: &[f64] = &[1., 2., 3.];
        let robj = Robj::from(src);
        let iter = <Real>::from_robj(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);

        /*
        let src: &[Bool] = &[TRUE, FALSE, TRUE];
        let robj = Robj::from(src);
        let iter = <Logical>::from_robj(&robj).unwrap();
        assert_eq!(iter.collect::<Vec<_>>(), src);
        */
    }
}
