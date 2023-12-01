#

![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering; //Used dor comparison of value sizes 

pub enum Comp { //Enumerate Comparison
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender { //Enumerate Gender
    Male,
    Female,
}

#[derive(Debug)] //Decorate the struct Person. Debug is an inbuilt trait. This statement will provoke impl Debug for Person; Metaprogramming
struct Person {
    name: String,
    age: u8,
}
struct Unit;
// A unit struct
//Have no state of their own but useful for
//implementing some trait.
//E.g. struct Global is a unit struct that can implement traits like Allocator
//std::fmt::Error is a unit struct that implements
//traits like Error

//A tuple struct
struct Pair(i32, f32); //No named fields but has fields

//A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct. Below Point is used as datatype in Rectangle
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn run() {

    //declare a variable of type Person and assign values.
    let person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:#?}", person); //{:#?} implies pretty debug print person. :? is for debug print and :#? is for pretty debug print

    // Instantiate a unit struct
    let _unit = Unit;//As simple as that. If Unit implements some trait, then _unit will demand those implementations

    //declare a Point
    let point = Point { x: 10.3, y: 0.4 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge,//left_edge here will be declared. If you use x:f32 only, x will be declared.
        y: top_edge,//top_edge here will be declared. If you use y:f32 only, y will be declared.
    } = point;
    dbg!(&left_edge,&top_edge);


    let _rectangle = Rectangle { //I used _ with rectangle to silence warning knowing that the variable is not used.
        //struct instantiation is an expression too as used below in Point
        top_left: Point {
            x: left_edge,//left_edge is available, thanks to the destructuring above
            y: top_edge,//top_edge is available, thanks to the destructuring above
        },
        bottom_right,
    };

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

//Let's work on user-defined traits. Traits enable us achieve polymorphism.
//We are designing Shape below for the purpose of 
//specifying all expected functions and methods in any struct that implements Shape.


// Define a trait for shapes
trait Shape {
    fn new(length: i32, width: i32, name: &'static str) -> Self;
    fn area(&self) -> i32;
    fn perimeter(&self) -> i32;
    fn set_length(&mut self, length: i32);
    fn get_length(&self) -> i32;
    fn set_width(&mut self, width: i32);
    fn get_width(&self) -> i32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}

// Define a struct for rectangles
#[derive(Default, Debug, Clone)]
struct Rect {
    length: i32,
    width: i32,
    name: &'static str,
}

// Implementation of methods for rectangles
impl Rect {
    // Default constructor for rectangles
    fn default() -> Self {
        Rect {
            length: 6,
            width: 7,
            name: "rectangle_name",
        }
    }
}

// Implement the Shape trait for rectangles
impl Shape for Rect {
    fn new(length: i32, width: i32, name: &'static str) -> Self {
        Rect { length, width, name }
    }

    fn area(&self) -> i32 {
        self.length * self.width
    }

    fn perimeter(&self) -> i32 {
        2 * (self.length + self.width)
    }

    fn set_length(&mut self, length: i32) {
        self.length = length;
    }

    fn get_length(&self) -> i32 {
        self.length
    }

    fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    fn get_width(&self) -> i32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Implement partial equality and ordering for rectangles
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

// Implement conversion from string to rectangle
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect { length, width, name }
    }
}

// Define a struct for circles
#[derive(Default, Debug, Clone)]
struct Circle {
    radius: i32,
    name: &'static str,
}

// Implementation of methods for circles
impl Circle {
    fn default() -> Self {
        Circle {
            radius: 5,
            name: "circle_name",
        }
    }
}

// Implement the Shape trait for circles
impl Shape for Circle {
    fn new(radius: i32, _width: i32, name: &'static str) -> Self {
        Circle { radius, name }
    }

    fn area(&self) -> i32 {
        // Assuming area is the only meaningful measure for a circle in this context
        // This could be replaced with a more accurate formula if needed
        (3 * self.radius * self.radius) / 2
    }

    fn perimeter(&self) -> i32 {
        // Perimeter of a circle is 2 * pi * radius
        (2.0 * std::f64::consts::PI * self.radius as f64) as i32
    }

    fn set_length(&mut self, radius: i32) {
        self.radius = radius;
    }

    fn get_length(&self) -> i32 {
        self.radius as i32
    }

    fn set_width(&mut self, _width: i32) {
        // Width is not applicable to circles, so do nothing
    }

    fn get_width(&self) -> i32 {
        0 // Width is not applicable to circles, so return 0
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Implement partial equality and ordering for circles
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

// Conversion from string to circle
impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Circle { radius, name }
    }
}


// Define a struct for triangles
#[derive(Default, Debug, Clone)]
struct Triangle {
    base: i32,
    height: i32,
    name: &'static str,
}

// Implementation of methods for triangles
impl Triangle {
    fn default() -> Self {
        Triangle {
            base: 4,
            height: 8,
            name: "triangle_name",
        }
    }
}

// Implement the Shape trait for triangles
impl Shape for Triangle {
    fn new(base: i32, height: i32, name: &'static str) -> Self {
        Triangle {
            base,
            height,
            name,
        }
    }

    fn area(&self) -> i32 {
        // Area of a triangle is (base * height) / 2
        (self.base * self.height) / 2
    }

    fn perimeter(&self) -> i32 {
        // Perimeter of a triangle is the sum of all three sides
        // In this case, assuming an equilateral triangle for simplicity
        3 * self.base
    }

    fn set_length(&mut self, base: i32) {
        self.base = base;
    }

    fn get_length(&self) -> i32 {
        self.base
    }

    fn set_width(&mut self, _height: i32) {
        // Width is not applicable to triangles, so do nothing
    }

    fn get_width(&self) -> i32 {
        0 // Width is not applicable to triangles, so return 0
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Implement partial equality and ordering for triangles
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

// Conversion from string to triangle
impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let base = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let height = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Triangle {
            base,
            height,
            name,
        }
    }
}

// Function to run and test the code
pub fn run2() {
    let rectangle1 = Rect::default();
    
    println!("{}", rectangle1.length);
    println!("{}", rectangle1.width);
    println!("{}", rectangle1.name);

    let rectangle2 = Rect::new(1, 3, "Rectangle2");
    let rectangle3 = Rect::from("4,5,Rectangle3");

    // Compare using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!("result1 = {:?}", result1);

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    // Compare using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);

    // Test Circle
    let circle1 = Circle::default();
    let circle2 = Circle::new(2, 0, "Circle2");
    let circle3 = Circle::from("3,3,Circle3");

    let circle_result1 = circle1.partial_cmp(&circle2);
    println!("circle_result1 = {:?}", circle_result1);

    let circle_result2 = circle1.le(&circle2);
    println!("circle_result2 = {:?}", circle_result2);

    let circle_result3 = circle2.eq(&circle3);
    println!("circle_result3 = {:?}", circle_result3);

    let circle_result4 = circle2.ne(&circle3);
    println!("circle_result4 = {:?}", circle_result4);

    // Test Triangle
    let triangle1 = Triangle::default();
    let triangle2 = Triangle::new(3, 0, "Triangle2");
    let triangle3 = Triangle::from("4,7,Triangle3");

    let triangle_result1 = triangle1.partial_cmp(&triangle2);
    println!("triangle_result1 = {:?}", triangle_result1);

    let triangle_result2 = triangle1.le(&triangle2);
    println!("triangle_result2 = {:?}", triangle_result2);

    let triangle_result3 = triangle2.eq(&triangle3);
    println!("triangle_result3 = {:?}", triangle_result3);

    let triangle_result4 = triangle2.ne(&triangle3);
    println!("triangle_result4 = {:?}", triangle_result4);
}
