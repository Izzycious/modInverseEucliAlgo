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
