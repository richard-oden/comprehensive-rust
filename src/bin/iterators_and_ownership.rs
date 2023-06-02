// Iterator returns item by reference (borrowing).
// fn main() {
//     let v: Vec<i8> = vec![10, 20, 30];
//     let mut iter = v.iter();

//     let v0: Option<&i8> = iter.next();
//     println!("v0: {v0:?}");
// }

// IntoIterator returns item by value.
// fn main() {
//     let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
//     let mut iter = v.into_iter();

//     let v0: Option<String> = iter.next();
//     println!("v0: {v0:?}");
// }

fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // word is borrowed
    for word in &v {
        print_type_of(word);
    }

    // word is not borrwed
    for word in v {
        print_type_of(word);
    }
}