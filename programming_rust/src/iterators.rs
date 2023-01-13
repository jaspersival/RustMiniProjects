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

struct MyIterator {
    values: Vec<i32>,
}

impl IntoIterator for MyIterator {
    type Item = i32;
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
