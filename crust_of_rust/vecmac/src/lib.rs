// A declarative macro defines a list of patterns where the source is
// syntaxtically valid rust (can be parsed), and the target is rust code
// replacing the pattern source, which means the pattern target must be rust
// that can be compiled.
#[macro_export] // `pub` for a macro.
macro_rules! avec {
    // `expr`: anything that can end with `;`.
    // the extra outter `{}` is ensure the expr is replaced by a single
    // block, not multiple expressions.

    // 1. use `$(pattern),+` to represent recurring patterns.
    //    `,`: a rust token separating two patterns, can also use, eg, `;`.
    //    `+`: 1 or more appearance, can also use `*`, `?`, etc.
    // 2. use the same `$(code)*` to generate code that repeats the same
    //    number of times as the repeatitive patterns.
    //    rust matches repeated pattern and code by using the same vari name.

    // This pattern matches:
    //     1. empty - avec![]
    //     2. N elements separated by `,` - avec![1, 2, 3]
    ($($element:expr),*) => {{
        // check that count is const
        const C: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C);
        $(vs.push($element);)*
        vs
    }};
    // This pattern matches:
    //      1. empty - avec![]
    //      2. N elements where each is followed by `,` - avec![1, 2,]
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};
    // This pattern matches:
    //      repeating element count times - avec![5; 3]
    ($element:expr; $count:expr) => {{
        // remember what macro really does is simply substution.
        // so here we assign the result of `$element` to a variable first,
        // and then repeat that result, not the expression, by cloning.
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};
    // First two patterns can be combined into one.
    // ($($element:expr),+ $(,)?) => {{
    //     let mut vs = Vec::new();
    //     $(vs.push($element);)+
    //     vs
    // }};
}

#[macro_export]
#[doc(hidden)]
///! The `count` macro returns the len of a repeated pattern it matches.
/// The 1st pattern calculate the len by creating a slides with `()` elements;
/// the 2nd pattern is used internally to replace element to `()`.
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_vec() {
        let v: Vec<u32> = avec!();
        assert!(v.is_empty());
    }

    #[test]
    fn single() {
        let v: Vec<u32> = avec![42];
        assert!(!v.is_empty());
        assert!(v.len() == 1);
        assert_eq!(v[0], 42);
    }

    #[test]
    fn double() {
        let v: Vec<u32> = avec![42, 43,];
        assert!(!v.is_empty());
        assert!(v.len() == 2);
        assert_eq!(v[0], 42);
        assert_eq!(v[1], 43);
    }

    #[test]
    fn repeat() {
        let v: Vec<u32> = avec![42; 2];
        assert!(!v.is_empty());
        assert!(v.len() == 2);
        assert!(v[0] == 42);
    }

    #[test]
    fn clone_2_nonliteral() {
        let mut y = Some(42);
        let x: Vec<u32> = avec![y.take().unwrap(); 2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}

/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42; "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;
