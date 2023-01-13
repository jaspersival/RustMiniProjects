use std::fmt::Debug;

#[test]
fn into_iterator_implementations() {
    fn dump<T, U>(t: T)
    where
        T: IntoIterator<Item = U>,
        U: Debug,
    {
        for u in t {
            println!("{:?}", u);
        }
    }

    let var = vec![1, 2, 3];
    dump(var);
}

struct MyIterator<T> {
    values: Vec<T>,
}

impl<T> IntoIterator for MyIterator<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

#[test]
fn loop_over_own_iterator_with_integers() {
    let my_iterator = MyIterator {
        values: vec![1, 2, 3],
    };
    for i in my_iterator {
        println!("{}", i);
    }
}

#[test]
fn loop_over_own_iterator_with_strings() {
    let my_iterator = MyIterator {
        values: vec!["foo", "bar", "baz"],
    };
    for i in my_iterator {
        println!("{}", i);
    }
}
#[test]
fn loop_over_own_iterator_with_floats() {
    let my_iterator = MyIterator {
        values: vec![1.5, 2.0, 2.5],
    };
    for i in my_iterator {
        println!("{}", i);
    }
}
