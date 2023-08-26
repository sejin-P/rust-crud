# RS-CRUD

## GOALS
1. Create a CRUD application to be familiar with Rust.
2. Use various features even it's not necessary, just for an educational purpose.
3. CRUD of a simple entity: User
4. CRUD of a simple entity: Post



## Processes
1. 08/19/2023: Connect to database(mysql), add GET user api
2. 08/20/2023: Fix compile errors in 1, Add POST, DELETE, PUT user api
3. 08/26/2023: CRUD of Post entity



## TODOS
1. Exception handling
2. ADD middlewares
3. Refactoring - divide dependencies into modules
4. Unit Tests
5. Use multithreading + async/await even it's not necessary
6. Add hot reload
7. Add auth



## Lesson Points

Closure - function-like construct that can capture its surrounding environment. 

Closures are similar to lambdas or anonymous functions in other programming languages. 

Closures in Rust have a unique feature: they can capture their environment in three ways: by reference, by mutable reference, or by value

You can use `move` keyword before the parameter list to force the closure to take ownership of the values it uses in the environment.

Especially in concurrency programming, you can use `move` keyword to transfer ownership of the values to the closure so that the closure can be executed in another thread.

```rust
use std::thread;

let message = "Hello from a thread!".to_string();

thread::spawn(move || {
    println!("{}", message);
}).join().unwrap();
```

