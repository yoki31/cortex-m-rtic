<div align="center"><img width="300" height="300" src="RTIC.svg"></div>
<div style="font-size: 6em; font-weight: bolder;" align="center">RTIC</div>

<h1 align="center">Real-Time Interrupt-driven Concurrency</h1>

<p align="center">A concurrency framework for building real-time systems</p>

# Preface

This book contains user level documentation for the Real-Time Interrupt-driven Concurrency
(RTIC) framework. The API reference is available [here](../../api/).

Formerly known as Real-Time For the Masses.

<!--There is a translation of this book in [Russian].-->

<!--[Russian]: ../ru/index.html-->

This is the documentation of v1.0.x of RTIC; for the documentation of version

* v0.5.x go [here](/0.5).
* v0.4.x go [here](/0.4).

## Is RTIC an RTOS?

A common question is whether RTIC is an RTOS or not, and depending on your background the
answer may vary. From RTIC's developers point of view; RTIC is a hardware accelerated
RTOS that utilizes the NVIC in Cortex-M MCUs to perform scheduling, rather than the more
classical software kernel.

Another common view from the community is that RTIC is a concurrency framework as there
is no software kernel and that it relies on external HALs.

---

{{#include ../../../README.md:7:47}}

{{#include ../../../README.md:48:}}
