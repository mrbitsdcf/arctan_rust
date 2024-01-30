# ARCTAN in Rust

Some tests for Rust language to compute arctan for an angle. 

Also, this code compares two Taylor methods to approximate arctan values and the builtin atan() function in Rust and with another languages like Python and FORTRAN.

How to run
---

Supposing we are trying to compute the angular diameter for the Sun, we need some data: 

Radius of the Sun = 696,340 km

Distance from the Earth = 150,000,000 km

We also know that diameter = 2 * radius, but the code will take care about this.

```
cargo run -- -d=150000000 -r=696340
```

Expected result:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/arctan -d=150000000 -r=696340`
Results using method 1:    0.5319492899698222
Results using method 2:    0.5319492899698222
Results with builtin ATAN: 0.5319492899698222
```

In Python, we can do something like this:

```
python -c "import math; print(((math.atan((696340*2)/150000000))*180)/math.pi)"
```

And the result will be **0.5319492899698222**

The Fortran code must be compiled to run.

```
gfortran arctan.f90 -o arctan
./arctan
```

The result will be:

```
 x             =    9.2845335602760315E-003
 atan_true     =   0.53194928816872267     
 taylor_arctan =   0.53194928816872256     
 error         =   -1.1102230246251565E-016
```

Comparing the results, in FORTRAN we have uncertainty starting from the eighth decimal place. Pretty good. 
