fn main() {
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    {
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area1(rect1)
        );
    }

    {
        let rect1=Rectangle{
            width:30,
            height:50
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area2(&rect1)
        );
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        // println!("rect1 is {}", rect1);
        println!("rect1 is {rect1:?}");
    }

    {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1);
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    {
        let sq= Rectangle::square(3);
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width>0
    }

    fn square(size: u32) -> Self {
        Self {
            width:size,
            height:size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// p1.distance(&p2);
// (&p1).distance(&p2);