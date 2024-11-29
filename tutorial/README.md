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

- uninitialized variable declaration (with default)
```rs
    let mut x:u32;
    x = 112; // initialization before use
```

- scoped variable declaration
```rs
    {
        let scoped_var = 42; // only accessible inside this block
    }
``` 

- variable declaration with ownership transfer
```rs
    let s1 = String::from("Rust");
    let s2 = s1; // ownership transferred to s2
```

- reference variable declaration
```rs
    let x = 5;
    let y = &x; // immutable reference
    let mut z = 10;
    let w = &mut z; // mutable reference
```

- using _ to ignore values
```rs
    let _ = 42; // ignore value
    let _temp = 112; // unused variable
```

- array and vector declaration
```rs 
    let array = [1,2,4];
    let vector = vec![1,1,2];
``` 

- default-initiated variable
```rs
    let value:i32 = Default:default(); // default is 0 for i32
```

- using let with Loops
```rs
    let counter = (0..112).collect::<Vec<_>>(); // collect values in a loop
```