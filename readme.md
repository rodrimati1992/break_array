Want indexing arrays to not work anymore?

Simply do this:

```compile_fail
use break_array;
let array=[0,1,2,3];
assert_eq!( array[0], 0 );
```
and marvel at the compiler error:
```text
  |
6 | assert_eq!( array[0], 0 );
  |                   ^ expected struct `break_array::MyType`, found integer

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
```

