# font-awesome

This is a library for getting font-awesome unicode codepoints
without using magic names. Presently (0.1.0), it only supports the free, bold
icons, and all of the typings are handmade. In the future, if time permits, it may get
more complicated.

Game engines may find this to be useful, so that these magic characters can be referenced
by name in code, rather than written out fully.

Additionally, it exposes a function which will provide a fat array of all of these in one.
That function is marked as `const`, and should probably be used to load a const if you intend to use that,
rather than putting them all on the stack.

This library is in no way endorsed or supported by FontAwesome directly, and I don't know if that's a problem.
