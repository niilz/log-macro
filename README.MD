# Simple Demo of an Attribute Macro

## What the `#[log]` macro does

If a function is annotated with the **log-macro** the function's name gets logged on entry and exit:

- Log output before the actual function body executes

```
> my_function_name
```

- Log output just before the function returns

```
< my_function_name
```

This is inspired by how logging can be realized with concepts from [Aspect Oriented Programming](https://en.wikipedia.org/wiki/Aspect-oriented_programming) (AOP).
Although this example is extremely simplified, it shows how additional logging can be added, merely through applying an attribute.
Very similar to how annotation-based logging works, like for example in Java.

## How the macro works

The _"annotated"_[\[1\]](#note-1) function is parsed into an `ItemFn`, using the [syn](https://docs.rs/syn/) crate.
It reads out the function signature (especially name and return type) and the body.
Than the code is reconstructed with [quote](https://docs.rs/quote/).
While an entry- and an exit-log-statement gets "injected" in the user-code.

Little interesting aside. It is important that the macro checks, whether the function returns anything else but the [Unit `()`](https://doc.rust-lang.org/std/primitive.unit.html) type.
Because if that is the case, the very last statment of the function has to be *quote*d before the exit-log-statment.

## How the repo is structured

The parent crate is **use-log-macro**. And it, like the name implies, uses the `#[log]` macro on some example functions, in it's `main.rs`.

The log-macro is an [attribute-macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros) and has to live in it's own crate.
Here it is nested as a library inside of **use-log-macro**, which references **log** by relative path declaration in it's `Cargo.toml`.

## What else could this style be use for

I think this pattern shows how it might be possible to inject all sorts of runtime measurement logic, simply by adding attributes to functions.
For example benchmark calculations.
Or reporting logic could be added and reduced in this very simple way. Maybe even by writing to a database.
All without cluttering the primary logic of the functions.

## Notes

<span id="note-1">[1] I know it is not called 'annotation' in Rust. But what would you say then: "The attributeapplied function...?"</span>
