fn largest<T: std::cmp::PartialOrd + Copy>(arr: &[T]) -> T {
    let mut res = arr[0];

    for &val in arr {
        if val > res {
            res = val;
        }
    }

    res
}

fn largest_reference<T: std::cmp::PartialOrd>(arr: &[T]) -> &T {
    let mut res = &arr[0];

    for val in arr {
        if val > res {
            res = val;
        }
    }

    res
}

fn largest_i32(arr: &[i32]) -> i32 {
    let mut res = arr[0];

    for &val in arr {
        if val > res {
            res = val;
        }
    }
    res
}

fn main() {
    let arr_char = vec!['b', 'c', 'B', 'C'];
    let arr_i32 = vec![10, 20, 0, 310];

    println!("largest char: {}", largest(&arr_char));
    println!("largest char: {}", largest_reference(&arr_char));
    println!("largest i32: {}", largest_i32(&arr_i32));
}

// impl TypeA {
//     // impl methods for TypeA.
// }

// impl<T> TypeA<T> {
//     // impl methods for generic type TypeA<T>
// }

// impl<T: Display> TypeA<T> {
//     // impl methods for generic type TypeA<T> where T has implemented trait Display.
// }

// impl TraitA for TypeA {
//     // impl a trait for a type.
// }

// impl<T: Display> TraitA for T {
//     // impl a trait for any type T where T has implemented trait Display.
// }