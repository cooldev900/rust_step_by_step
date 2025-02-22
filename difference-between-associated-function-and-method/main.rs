struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Associated function
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Methods
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut rect = Rectangle::new(3.0, 4.0);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());
    rect.scale(2.0);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square: {}", rect.is_square());
}
 
//  Associated functions are defined using the  impl  block, but they don't take  self  as a parameter. They are called using the type name, not an instance of the type. 
//  Methods are defined using the  impl  block and take  self  as a parameter. They are called using an instance of the type. 
//  Associated functions are used to create new instances of a type, while methods are used to perform operations on instances of a type. 
//  In the example above, the  new  function is an associated function that creates a new instance of the  Rectangle  struct, while the  area ,  perimeter ,  is_square , and  scale  functions are methods that operate on instances of the  Rectangle  struct. 
//  Associated functions are often used as constructors for a type, while methods are used to perform operations on instances of a type. 
//  In the example above, the  new  function is an associated function that creates a new instance of the  Rectangle  struct, while the  area ,  perimeter ,  is_square , and  scale  functions are methods that operate on instances of the  Rectangle  struct. 
//  Associated functions are defined using the  impl  block, but they don't take  self  as a parameter. They are called using the type name, not an instance of the type. 
//  Methods are defined using the  impl  block and take  self  as a parameter. They are called using an instance of the type. 
//  Associated functions are used to create new instances of a type, while methods are used to perform operations on instances of a type. 
//  In the example above, the  new  function is an associated function that creates a new instance of the  Rectangle  struct, while the  area ,  perimeter ,  is_square , and  scale  functions are methods that operate on instances of the  Rectangle  struct. 
//  Associated functions are often used as constructors for a type, while methods are used to perform operations on instances of a type