// run-rustfix
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::iter_cloned_collect)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::redundant_closure_for_method_calls)]

fn main() {
    let _: Vec<i8> = vec![5_i8; 6].iter().map(|x| *x).collect();
    let _: Vec<String> = vec![String::new()].iter().map(|x| x.clone()).collect();
    let _: Vec<u32> = vec![42, 43].iter().map(|&x| x).collect();
    let _: Option<u64> = Some(Box::new(16)).map(|b| *b);
    let _: Option<u64> = Some(&16).map(|b| *b);
    let _: Option<u8> = Some(&1).map(|x| x.clone());

    // Don't lint these
    let v = vec![5_i8; 6];
    let a = 0;
    let b = &a;
    let _ = v.iter().map(|_x| *b);
    let _ = v.iter().map(|_x| a.clone());
    let _ = v.iter().map(|&_x| a);

    // Issue #498
    let _ = std::env::args().map(|v| v.clone());
}
