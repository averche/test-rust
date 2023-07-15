#![feature(portable_simd)]
#![feature(test)]

extern crate test;

mod simd_test;

fn main() {
    derive_builder_test();
    simd_test::t();
}

#[macro_use]
extern crate derive_builder;

#[derive(Default, Builder, Debug)]
#[builder(setter(into))]
struct Channel {
    token: i32,
    special_info: i32,
    // .. a whole bunch of other fields ..
}

fn derive_builder_test() {
    // builder pattern, go, go, go!...
    let ch = ChannelBuilder::default()
        .special_info(42u8)
        .token(19124)
        .build()
        .unwrap();
    println!(
        "{:?} token: {} special_info: {}",
        ch, ch.token, ch.special_info
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stuff() {
        basics();
        animal_test();
    }

    fn basics() {
        let i1: u64 = 17;
        let i2: u64 = 18;

        match i1.cmp(&i2) {
            std::cmp::Ordering::Less => println!("asd1"),
            std::cmp::Ordering::Equal => println!("asd2"),
            std::cmp::Ordering::Greater => println!("asd3"),
        }

        let str1: String = String::from("hello");
        let str2: String = String::from("world");

        let str3: String = str1 + &str2;

        print_str_ref(&str3);
        print_str_ref(&str3);
        print_str(str3); // moved

        let mut arr1 = [5, 1, 2, 3, 4];

        arr1.sort();

        for i in arr1.iter() {
            println!("arr {}", i);
        }

        let mut vec1: Vec<i32> = vec![1, 2, 3, 4];

        while let Some(i) = vec1.pop() {
            println!("i = {}", i);
        }

        let dish = ("Bacon", "Eggs");

        if let ("Bacon", b) = dish {
            println!("Bacon is served with {}", b);
        } else {
            println!("No bacon will be served");
        }
    }

    fn print_str_ref(x: &str) {
        println!("str {}", x);
    }

    fn print_str(x: String) {
        println!("str {}", x);
    }

    fn animal_test() {
        let cat1: Box<dyn Animal> = Box::new(Cat {
            name: "cat1".to_string(),
        });
        let dog1: Box<dyn Animal> = Box::new(Dog {
            name: "dog1".to_string(),
        });
        let dog2: Box<dyn Animal> = Box::new(Dog {
            name: "dog2".to_string(),
        });

        let cat3 = Cat {
            name: "cat3".to_string(),
        };
        let cat4 = Cat {
            name: "cat4".to_string(),
        };

        print_animal(&cat3);
        print_animal(&cat4);
        print_animal_t(&cat4);

        let mut animals: Vec<Box<dyn Animal>> = vec![cat1, dog1, dog2];
        animals.push(Box::new(cat3));
        animals.push(Box::new(cat4));

        print_animals(&animals);
    }

    // TRAITS //

    trait Animal {
        fn speak(&self, prefix: &str);
    }

    struct Cat {
        name: String,
    }

    impl Animal for Cat {
        fn speak(&self, prefix: &str) {
            println!("{}: {}: meow!", prefix, self.name);
        }
    }

    struct Dog {
        name: String,
    }

    impl Animal for Dog {
        fn speak(&self, prefix: &str) {
            println!("{}: {}: meow!", prefix, self.name);
        }
    }

    fn print_animals(animals: &[Box<dyn Animal>]) {
        for a in animals {
            a.speak("animals");
        }
    }

    fn print_animal_t<T: Animal>(animal: &T) {
        animal.speak("animal_t");
    }

    fn print_animal(animal: &dyn Animal) {
        animal.speak("animal");
    }
}
