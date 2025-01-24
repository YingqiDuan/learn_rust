use generictypes::NewsArticle;
use std::fmt::Display;

fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);
    }

    {
        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}");
    }

    {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        // let wont_work = Point {x: 5, y: 4.0 };

        println!("integer is {}", integer.x);
    }

    {
        let both_integer = Point2 { x: 5, y: 10 };
        let both_float = Point2 { x: 1.0, y: 4.0 };
        let integer_and_float = Point2 { x: 5, y: 4.0 };
    }

    {
        let p1 = Point3 { x: 5, y: 10.0 };
        let p2 = Point3 { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    {
        enum Option_i32 {
            Some(i32),
            None,
        }

        enum Option_f64 {
            Some(f64),
            None,
        }

        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }

    {
        use generictypes::{NewsArticle, Summary, Tweet};

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
    }

    {
        let s = 3.to_string();
    }

    // { // borrowed value does not live long enough
    //     let r;                      // ---------+-- 'a
    //                                 //          |
    //     {                           //          |
    //         let x = 5;              // -+-- 'b  |
    //         r = &x;                 //  |       |
    //     }                           // -+       |
    //                                 //          |
    //     println!("r: {r}");         //          |
    // }                               // ---------+

    {
        let x = 5; // ----------+-- 'b
                   //           |
        let r = &x; // --+-- 'a  |
                    //   |       |
        println!("r: {r}"); //   |       |
                            // --+       |
    } // ----------+

    {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);

        // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        //     x
        // }

        // // dangling reference
        // fn longest2<'a>(x: &str, y: &str) -> &'a str {
        //     let result = String::from("really long string");
        //     result.as_str()
        // }

        fn longest2(x: &str, y: &str) -> String {
            String::from("really long string")
        }

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }

        // let string1 = String::from("long string is long");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str());
        // }
        // println!("The longest string is {}", result);
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("first sentence: {}", i.part);

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }

        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }

    {
        let s: &'static str = "I have a static lifetime.";

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
