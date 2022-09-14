# hail
*An experimental systems-level programming language to replace C.*

> **please note**: Hail is *very* unfinished and probably doesn't work.

## about hail
Hail is inspired by C, C++, Go, Jai, Odin, Rust, Zig and a few others.  It is not memory managed at all (unlike Rust and Go).  While Hail aims to have the same capabilities as C, it doesn't allow any implicit conversions.

For example, in C:

```c
int main() {
    int my_variable = 42;
    free(my_variable); // implicitly converts `int` -> `void*`
}
```

In Hail, the equivalent code must contain an explicit conversion using the **`as`** keyword:

```hail
publ fun main() -> int32 {
    val my_variable = 42;
    free(my_variable as *mut uint8);
}
```

By default, Hail warns about possible unsafe code, but doesn't *disallow* it.  These warnings can be disabled with **`#[..]`** attributes, similar to Rust.

For example:
```hail
publ fun main() -> int32 {
    println("Hello, world!");
    // no return
    // warning: function never returns a value
}
```

This can make writing code faster, for example, the **`int32`** type implements the **`Default`** trait provided by Hail (it returns **`0`**).  If a function expects a return value but nothing is returned (like above), and the type the function returns implements **`Default`**, then the function will return the default value for that type.

The previous example implicitly returns **`int32::default()`**.  ***Implicit returns for types that do not implement `Default` will likely cause undefined behavior.***