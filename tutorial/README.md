# Rust declaration variable

- immutable variable declaration
```rs
    let x = 112; // immutable variable
```

- mutable variable declaration
```rs
    let mut y = 112; // mutable variable
    y = 113; // value changed
```

- type-annotated variable declaration
```rs
    let z:f64 = 3.14; // explicit type annotation
```

- destructuring declaration
```rs
    let (a,b) = (112,113); // destructure tuple
    let Point {x,y} = Point {x:5,y:10};
```

