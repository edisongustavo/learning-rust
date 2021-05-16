use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

use rand::{Rng, thread_rng};

use hello_rust::agenda;

mod pig_latin;

// use pig_latin;


const PI: f64 = 3.14;

fn guessing_game() {
    println!("Guess the number");
    let secret = thread_rng().gen_range(1..10);
    println!("Secret is {}", secret);

    println!("What's your guess?");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Right!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}

fn bar(mut x: i32) {
    x += 1
}

fn mutating_variables() {
    let x = 5;
    println!("The value of x is: {}", x);
    bar(x);
    println!("The value of x is: {}", x);
}

fn main() {
    // guessing_game()
    mutating_variables();
    types();
    statements_and_expressions();
    booleans();
    loops();
    ownership();
    onwership_and_functions();
    slices();
    structs();
    enums();
    vectors();
    strings();
    hashmaps();
    exercises_8_3();
    generics();
}

fn generics() {
    fn largest_with_index<T: PartialOrd>(list: &[T]) -> &T {
        if list.is_empty() {
            panic!("Can't find largest item of empty list");
        }

        let mut largest = 0;
        for i in 0..list.len() {
            if list[i] > list[largest] {
                largest = i;
            }
        }
        return &list[largest];
    }
    fn largest_with_reference<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = &item;
            }
        }
        return largest;
    }

    let number_list = vec![1, 2, 3, 0, 2];
    println!("Largest from {:?} is {}", number_list, largest_with_index(&number_list));
    println!("Largest from {:?} is {}", number_list, largest_with_reference(&number_list));


    fn longest<'x>(a: &'x str, b: &'x str) -> &'x str {
        if a.len() > b.len() {
            a
        } else {
            b
        }
    }
    let a = "foo";
    let b = "foobar";
    println!("Longest of {} and {} is {}", a, b, longest(a, b));

    let string1 = String::from("a string");
    let result;
    {
        let string2 = String::from("another string");
        // result = longest(string1.as_str(), string2.as_str());
        let result_inner = longest(string1.as_str(), string2.as_str());
        result = String::from(result_inner);

    }
    println!("Longest string is {}", result);
}

fn exercises_8_3() {
    {
        /*
        Given a list of integers, use a vector and return:
        - the mean (the average value),
        - median (when sorted, the value in the middle position),
        - mode (the value that occurs most often; a hash map will be helpful here)
        of the list.
        */
        #[derive(Debug)]
        struct Values {
            mean: f64,
            median: i32,
            mode: i32,
        }
        fn calculate_values(v: &Vec<i32>) -> Values {
            if v.is_empty() {
                panic!("Can't calculate values on empty vector");
            }
            let sum = v.iter().sum::<i32>() as f64;
            let mean = sum / v.len() as f64;

            let mut sorted_vector = v.clone();
            sorted_vector.sort();
            let median = sorted_vector[v.len() / 2];

            let mut count_ocurrences = HashMap::new();
            for elem in v {
                *count_ocurrences.entry(elem).or_insert(0usize) += 1usize;
            }
            let mode = **count_ocurrences.iter().reduce(
                |lhs, rhs|
                    {
                        if lhs.1 > rhs.1 {
                            lhs
                        } else {
                            rhs
                        }
                    }
            ).expect("").0;

            return Values {
                mean, median, mode
            }
        }
        let v = vec![1, 2, 3, 3, 3, 4, 5, 6, 6, 6, 6, 6, 0, -1];
        let calculated_values = calculate_values(&v);
        println!("vector: {:?}", v);
        println!("calculated_values: {:?}", calculated_values);
    }

    {
        let s = pig_latin::convert_words("    If is_extended is true, the iterator is over the extended grapheme clusters; otherwise, the iterator is over the legacy grapheme clusters. UAX#29 recommends extended grapheme cluster boundaries for general processing.");
        println!("pig latin: {}", s);
    }

    {
        // agenda::agenda();
    }
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 52);

    for (team, score) in &scores {
        println!("Team {} has score {}", team, score);
    }

    scores.insert("Green", 20);
    for (team, score) in &scores {
        println!("Team {} has score {}", team, score);
    }

    scores.entry("Red").or_insert(30);
    for (team, score) in &scores {
        println!("Team {} has score {}", team, score);
    }

    fn count_words(text: &str) -> HashMap<&str, i32> {
        let mut map = HashMap::new();
        for word in text.split_ascii_whitespace() {
            let current_count = map.entry(word).or_insert(0);
            *current_count += 1;
        }
        return map
    }
    let text = "Hello people of arcadia. How have, of all people, you been doing?";
    let counts = count_words(text);
    println!("Counts of {} is: {:?}", text, counts);
}

fn strings() {
    let s = "नमस्ते".to_string();
    let chars = s.chars();
    for c in chars {
        println!("Char is {}", c);
    }
    // chars[0];

    println!("{}", "İ".to_lowercase());
    // assert_eq!("İ".to_lowercase(), "i");
}

fn vectors() {
    let v1 = vec![1, 2, 3, 4];

    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);

    assert_eq!(v1, v2);

    let elem0 = &v1[0];
    let elem1 = v1.get(0).expect("Waaaat");
    assert_eq!(elem0, elem1);

    let string_vector= vec!["Foo", "Bar", "Other bar"];
    for name in string_vector {
        println!("Name is {}", name);
    }

    let mut mutable_string_vector = vec!["Foo", "Bar", "Other bar"];
    let mut other = mutable_string_vector.iter_mut().map(|s| String::from(*s)).collect::<Vec<String>>();
    for name in &mut other {
        let mut bar = &*name;
        // *name += "-appended";
        println!("Name is: {}", bar);
    }

    for name in other {
        println!("Name is {}", name);
    }
}

fn enums() {
    enum IpAddressKind {
        V4, V6
    }

    let v4 = IpAddressKind::V4;

    #[derive(Debug)]
    enum IpAddress {
        V4(u8, u8, u8, u8),
        V6(String),
        // V8,
    }

    let v4 = IpAddress::V4(255, 255, 255, 255);
    println!("IP (v4) is: {:?}", v4);
    println!("IPv4 ({:?}) is_private: {}", v4, v4.is_private());

    impl IpAddress {
        fn is_private(&self) -> bool {
            match self {
                IpAddress::V4(oct1, oct2, oct3, oct4) => oct1 == &127u8,
                IpAddress::V6(s) => false,
                _ => false
            }
            // if self == IpAddress::V4 {
            // }
            // return self.0 == 127;
        }
        fn is_private_if_let(&self) -> bool {
            if let IpAddress::V4(oct1, oct2, oct3, oct4) = self {
                return oct1 == &127u8;
            }
            return false;
        }
    }

    //---------------------------------------
    // let foo = Some(10);
    fn show_ip(ip: &IpAddress) {
        println!("IPv4 ({:?}) is_private: {}", ip, ip.is_private());
    }
    show_ip(&IpAddress::V4(127, 0, 0, 0));

    //--------------
    fn maybe_add(v: Option<i32>) -> Option<i32> {
        match v {
            None => None,
            Some(value) => Some(value + 1),
        }
    }
    println!("maybe add: {}", maybe_add(Some(10)).expect("Carai!"));
}

fn structs() {
    struct Foo {
        string_field: String,
        float_field: f64,
        integer_field: i32,
    }

    let foo = Foo {
        string_field: String::from("test"),
        float_field: 1.0,
        integer_field: 42
    };
    println!("Foo: foo.string_field = {}", foo.string_field);

    fn build_foo(string_field: &str, float_field: f64) -> Foo {
        return Foo {
            string_field: String::from(string_field),
            float_field,
            integer_field: 1,
        }
    }
    build_foo("bar", 42.0);

    impl Foo {
        fn bar(&self) {
            println!("Foo: self.string_field = {}", self.string_field);
        }
        // fn foobar(&this) {
        //     println!("Foo: self.string_field = {}", this.string_field);
        // }
    }

    impl Foo {
        fn foobar(&self) {
            println!("Foo bar: self.string_field = {}", self.string_field);
        }
    }

    foo.bar();
    foo.foobar();
}

fn slices() {
    fn first_word_index(s: &str) -> usize {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            if b == b' ' {
                return i;
            }
        }
        if s != "" {
            return 0;
        }
        panic!("No first word!");
    }
    fn first_word_slice(s: &str) -> &str {
        for (i, &b) in s.as_bytes().iter().enumerate() {
            if b == b' ' {
                return &s[0..i];
            }
        }
        if s != "" {
            return s;
        }
        panic!("No first word!");
    }
    println!("First word end index: {}", first_word_index(&String::from("Hello world")));
    let mut my_string = String::from("Hello world");
    println!("First word end index: {}, the word is {}", first_word_index(&my_string), &my_string[0..first_word_index(&my_string)]);

    let s = "Hello world";
    let word = &s[0..5];
    println!("Word is {}", word);
    println!("First word end index: {}, the word is {}", first_word_index(&s), &s[0..first_word_index(&s)]);

    println!("First word slice: {}", first_word_slice(s));

    // This panics!
    // let past_end = &s[5..99];
    // let past_end2 = &my_string[5..99];
    // println!("Past end: {}", past_end);
    // println!("Past end2: {}", past_end2);

    // let parts = s.split(" ");
    // // parts.next();
    // // return parts[0];
    // let parts = s.split_ascii_whitespace();
    //
    // let (range, dims) = [0..10];
    // for value in range.iter() {
    //     println!("Value {}", value);
    // }
}

fn onwership_and_functions() {
    println!("########");
    println!("onwership_and_functions()");
    println!("########");

    let s = String::from("My string");
    take_ownership(s);
    // println!("s has len: {}", s.len());

    fn take_ownership(some_string: String) {
        println!("Some string: {}", some_string);
    }

    //------------------------------------------------------

    let s2 = String::from("My string 2");
    println!("s2 has len: {}", s2.len());
    // take_ownership_returning(s2);
    let s2 = take_ownership_returning(s2);
    println!("s2 has len: {}", s2.len());

    let s2 = take_ownership_returning(String::from("My other string"));
    println!("s2 has len: {}", s2.len());

    fn take_ownership_returning(some_string: String) -> String {
        println!("The string '{}' has len: {}", some_string, some_string.len());
        return some_string;
    }

    //------------------------------------------------------
    fn calculate_length(s: &String) -> usize {
        return s.len();
    }
    let s = String::from("My string");
    println!("Length is {}", calculate_length(&s));

    //------------------------------------------------------
    fn mutate_string(s: &mut String) {
        s.push_str(" -> The mutation!!!");
    }
    let mut s = String::from("My string!");
    mutate_string(&mut s);
    take_ownership(s);

    //------------------------------------------------------
    fn dangle() -> String {
        let s = String::from("A string");
        return s;
    }
    let s = dangle();
}

fn ownership() {
    let s = "hello";

    {
        let foo = "Hello bar bolinha";
        println!("Hello {}, you size is: {}", foo, foo.len());
    }

    let mut s = String::from("Foo bar");
    s.push_str(", but pushing later");
    println!("String is: {}", s);

    let s2 = s;
    println!("Moved string is: {}", s2);

    let mut s3 = s2.clone();
    s3.push_str(", and now pushing on clone.");

    // s[0] = 'G';
    println!("Moved string is: {}", s2);
    println!("Cloned string is: {}", s3);
}

fn loops() {
    let x = loop {
        break 10
    };
    println!("Value of x is: {}", x);

    let x = while true {
        break;
    };

    let arr = [1, 2, 3, 4, 5];
    for elem in arr.iter() {
        println!("Value of x is: {}", elem);
    }

    for i in 1..10 {
        println!("Value of i is: {}", i);
    }

}

fn booleans() {
    let very_long_condition = true;
    let number = if very_long_condition {
        let foo = 10;
        foo + 20
    } else {
        6
    };
}

fn statements_and_expressions() {
    fn foo() -> i32 {
        if false {
            return 10;
        }
        5
    }

    let x = foo();
    println!("Value of x is: {}", x);
}

fn types() {
    let mut x: u32 = 0;
    println!("The value of x is: {}", x);
    // x -= 1; // Overflow (panic)
    println!("The value of x is: {}", x);

    let mut x: u8 = 255;
    // x += 1; // Overflow (panic)
    // wrapping_add(x, 1);
    println!("The value of x is: {}", x);

    //Tuple
    let t = (1, 2.0, "foobar", 'ß');
    let s = t.2;

    //Array
    let arr = [1, 2, 3, 4];
    let arr_float = [1.0, 2.0, 3., 4.];
    let element = arr_float[0];
    println!("Element is {}", element);

    let arr = ["a"; 10];
    println!("Element is {}", arr[2]);
}
