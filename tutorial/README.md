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

- destructuring variable declaration
```rs
    let (a,b) = (112,113); // destructure tuple
    let Point {x,y} = Point {x:5,y:10};
```

- constant variable declaration
```rs
    const MAX_POINTS: u32 = 112_112;
```

- static variable declaration
```rs
    static GREETING:&str = "Hello, Rust";
```

- shadowing variable
```rs
    let x = 5;
    let x = x + 1; // shadows the previous `x`
```