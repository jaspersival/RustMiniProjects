use std::collections::HashMap;

#[test]
fn play_with_closures() {
    type BoxedCallBack = Box<dyn Fn(&str) -> bool>;

    struct Printers {
        printers: HashMap<String, BoxedCallBack>,
    }
    impl Printers {
        fn new() -> Printers {
            Printers {
                printers: HashMap::new(),
            }
        }
        fn add<C>(&mut self, printer_name: &str, printer: C)
        where
            C: Fn(&str) -> bool + 'static,
        {
            self.printers
                .insert(printer_name.to_string(), Box::new(printer));
        }
        fn call(&self, printer_name: &str, text: &str) -> bool {
            match self.printers.get(printer_name) {
                None => false,
                Some(callable) => callable(text),
            }
        }
    }

    let printer1 = |name: &str| {
        println!("Hello {}!", name);
        return true;
    };

    let printer2 = |candy: &str| {
        println!("I want {}!", candy);
        return true;
    };

    let mut printers = Printers::new();
    printers.add("say_hello", printer1);
    printers.add("candy_wish", printer2);

    let return_say_hello = printers.call("say_hello", "Jasper");
    let return_candy_wish = printers.call("candy_wish", "chocolate");
    assert!(return_say_hello);
    assert!(return_candy_wish)
}
