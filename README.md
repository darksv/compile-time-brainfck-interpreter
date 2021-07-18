Warning!
=======

This repository contains some wildly unsafe Rust code that uses many unstable `const fn` features like compile-time heap allocation or operations on raw pointers.
These features were (ab)used to build a compile-time brainf*ck interpreter.
**Do not try this at home!** Similar effect can be achieved much easier on stable compiler with just a procedural macro!
