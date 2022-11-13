# modInverseEucliAlgo

The Aim is to understand how cryptography works and to do so, the basic understanding of **Modular Arithemetic** is needed. Upon understanding this will we be able to write a distinct programme that calculates the **modular multiplicative inverse** of a number using the Euclidean Algorithm.

## This is a workthrough to calculate the modular multiplicative inverse of a number using Euclidean Algorithm

**First going through the Modular Maths:**

When the division of two integers an equation that looks like the following:

```
    A/B = Q reminder R

```

where :

- A is the dividend
- B is the divisor
- Q is the quotient
- R is the reminder

On the reminder when we divide `A/B` operator is called **MOD**.

### Converting a positive integer in a Modular Arithemetic

The Euclidean Algorithm is a set of instructions for finding the greatest common divisor
of any two positive integers.

```
    a = bq + R
```

The euclidean Algorithm shows that given two integers `0 < b < a`, where we can say the making of the repeated division to a obtain a series of division equation which will be eventually terminated at the reminder of `0`.

Shows:

```
    a = bq1 + r1, 0 < r1 < b

    b = r1q2 + r2, 0 < r2 < r1

    r = r2q3 + r3, 0 < r3 < r3

    rj−2 = rj−1qj + rj , 0 < rj < rj−1

    rj−1 = rjqj+1
```

A quick exaample on the Implementation
Finding the gcd of(7469, 2464) using the Euclidean Algorithm:

```
//from the equation a = bq + R

    7469 = 2464q + R
    7469 = 2464(3) + 77
    2464 = 77(32) + 0

 gcd(7467, 2464) = 32
```

## MULTIPLICATIVE INVERSE (EUCLIDEAN ALGORITHM)

Given two integers `0 < b < a`, considering the euclidean equation which will yield the `gcd(a,b) = r`.

Rewriting all the equation except the last equations by solving for the reminders.

```
    r = a - bq

    r2 = b - rq2

    r3 = r1 - r2q3

    Therefore, rj−1 = rj−3 − rj−2qj−1

                rj = rj−2 − rj−1qj
```

The last of the equation `rj = rj−2 − rj−1qj` will replace `rj-1` with its expression in terms of `rj-3` and `rj-2` from the equation immediatialy above it, continueing the process successively replacing `rj-2`, `rj-3` and so till a final equation is obtained.

```
    rj = ax + by
```
