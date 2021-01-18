# tib
[t]ib [i]s not TI-[B]asic

© Jayden Milne 2020

tib is an interpreter for the dialect of TI-BASIC found on the TI-84, written in
Rust. It aims to be bug-for-bug compatible, anything that works on a calculator 
is fair game for tib. If you find something that doesn't work, please open an 
issue and let me know!

## Features
1. Read-Eval-Print Loop (REPL)
2. Written in cross-platform Rust

## Goals
The tib project's goals are
1. **Any valid TI-BASIC program is a valid tib program**

   This includes any syntatic oddities that TI calculators have, except for
   obvious bugs `Disp "HELLO:Disp (1(2(3` is a valid program.

   Those who prefer a stricter no-nonsense subset of TI-BASIC can pass the `-s`
   or `--strict` paramater to disallow such shenanigans
2. **Implement the parts of TI-BASIC that are useful as a programming language**

    While an effort will be made to implement as many functions TI-BASIC
    provides, some things that just aren't useful won't be implemented, like
    yvars and plotting coordinates.


## Non-Goals

1. **Replicate timing exactly**

   The TI-BASIC interpreter on TI-84s is hilariously slow. tib is not 
   particularly optimized, but replicating the timing of a calculator perfectly
   would be a tall order. Sorry, your delay loops will be super quick. Welcome
   to the 21st century.

2. **Replicate the TI-84's floating point behavior**

    Every number in tib is represented by rust's `f64`, so the exact behavior of
    that is up to your machine. Replicating the intricacies of how floats are
    handled in the Zilog Z80 does not sound fun to me.

3. **Replace a TI-84**

   There are some features from the TI-84 that probably will never happen, such
   as plotting graphs or yvars, since I don't see how those are useful for a 
   programming language. One day, if you are lucky, you may get to draw things
   to the graph screen, but that's it.

4. **Fail in exactly the same way**

   The goal is that every valid TI-BASIC program is a valid tib program. Note 
   this constraint says nothing about the inverse or the converse. An invalid 
   TI-BASIC program is not necessarily an invalid tib program, and tib may let 
   you get away with something you can't do on a calculator. (please let me know 
   if you find differences though, so I can try to mitigate them!)
   
   A best-effort attempt will be made to fail in the same way, but once you 
   start crashing a calculator, you are entering the realm of Undefined 
   Behavior™ and I make no guarantees what tib will do.

