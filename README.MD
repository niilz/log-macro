# Simple Demo of an Attribute Macro

## What the `#[log]` macro does

If a function is annotated with this **macro** the function's name gets logged before the actual function body executes.

This is inspired by how logging can be realized with concepts from [Aspect Oriented Programming](https://en.wikipedia.org/wiki/Aspect-oriented_programming) (AOP).
Although this example is extremely simplified and only logs function entrences, it shows how additional logging can be added through attributes.
Very similar to annotation-based logging, like for example in Java.

## How the repo is structured

The parent crate is **use-log-macro**. And it, like the name implies, uses the `#[log]` macro on some example functions, in it's `main.rs`.

The log-macro is an [attribute-macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros) and has to live in it's own crate.
Here it is nested as a library inside of **use-log-macro**, which references **log** by relative path declaration in it's `Cargo.toml`.

## What would be a nice extension

Currently the the start of a function is logged in the form:

```
> my_function_name
```

But would it be awesome to maybe also log the end of the function, like:

```
< my_function_name
```

Although that is totally possible, it requires a bit more advanced parsing.
Because we would first have to check whether the function returns a type.
If that is the case, we have to "inject" the print statement before the return value is [quote](https://docs.rs/quote/)d.

Also I think this pattern shows how it might be possible to inject all sorts of runtime measurement logic, simply by adding attributes to functions.
