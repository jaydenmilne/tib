# tib
[t]ib [i]s not TI-[B]asic

© Jayden Milne 2018, all rights reserved.

tib is an interpreter for a subset of TI-BASIC with a few convenience features
added written in Rust.

**tib does not aim to be a calculator.** It can certainly be used as one if you
want, but it is focused on being an interpreter for a programming language. For
this reason some feautures are not supported, such as the graph view, yvars,
plotting functions and others. If you desire those features there are plenty of
TI calculator emulator projects available.

## Goals
The tib project's goals are
1. **Any valid TI-BASIC program is a valid tib program**

   This includes any syntatic oddities that TI calculators have, except for
   obvious bugs `Disp "HELLO:Disp (1(2(3` is a valid program.

   Those who prefer a stricter no-nonsense subset of TI-BASIC can pass the `-s`
   or `--strict` paramater to disallow such shenanigans
2. **An invalid TI-BASIC program is not necessarily an invalid tib program**

   This means that things that might crash your calculator will run fine under
   tib. For example, `{"HELLO","TI-84"}` will throw an `ERR:DATA TYPE` error on
   a calculator, but tib will happily except it.

   Those who so desire can pass the `-e` or `--emulate` paramater for a more
   authentic calculator experience.
3. **Implement the parts of TI-BASIC that are useful as a general-purpose programming language**

    While an effort will be made to implement as many functions TI-BASIC
    provides, some things that just aren't useful won't be implemented. For
    instance, complex number support probably won't happen, as it isn't
    generally very useful in programming. yvars likely fall into the same
    category
