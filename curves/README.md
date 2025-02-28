# Curve implementations

This repository contains implementations of some elliptic curves. The curve API implemented here matches the curve traits defined in [ark-ec](https://github.com/arkworks-rs/algebra/blob/master/ec/src/lib.rs).

### BN-183 and related curves

* [`ark-bn183`](bn183): Implements a Barreto-Naehring pairing-friendly curve (embedding-degree 12) with a 183 bit prime group order
* [`ark-ed-on-bn183`](ed_on_bn124): Implements a Twisted Edwards curve atop the scalar field of bn183

### BN-124 and related curves

* [`ark-bn124`](bn124): Implements a Barreto-Naehring pairing-friendly curve (embedding-degree 12) with a 124 bit prime group order
* [`ark-ed-on-bn124`](ed_on_bn124): Implements a Twisted Edwards curve atop the scalar field of bn124

### ED-181 and related curves

* [`ark-ed181`](ed181): Implements a Galbraith-McKee-Valenca pairing-friendly curve (embedding-degree 6) with a 181 bit prime group order. This is the 'Edwards' curve in libsnark.
* [`ark-ed-on-ed181`](ed_on_ed181): Implements a Twisted Edwards curve atop the scalar field of ed181

### ED-97 and related curves

* [`ark-ed97`](ed97): Implements a Galbraith-McKee-Valenca pairing-friendly curve (embedding-degree 6) with a 97 bit prime group order.
* [`ark-ed-on-ed97`](ed_on_ed97): Implements a Twisted Edwards curve atop the scalar field of ed97

### ED-61 and related curves

* [`ark-ed61`](ed61): Implements a Galbraith-McKee-Valenca pairing-friendly curve (embedding-degree 6) with a 61 bit prime group order.
* [`ark-ed-on-ed61`](ed_on_ed61): Implements a Twisted Edwards curve atop the scalar field of ed61

### ED-58 and related curves

* [`ark-ed58`](ed58): Implements a Galbraith-McKee-Valenca pairing-friendly curve (embedding-degree 6) with a 58 bit prime group order. 
* [`ark-ed-on-ed58`](ed_on_ed58): Implements a Twisted Edwards curve atop the scalar field of ed181
