# Certums

Translated-
Certum - Fixed
Quarta - Quarter
Dimidium - Half
Acute - Sharp

This is a simple numeric extension in rust that implements low-level fixed-sized fixed-point numbers
as well as extensions and utilities for them in many different forms.

The goal is to have a flexible and high-precision number system for tasks that rely largely on fractional components over supporting whole-number values

Certums and their variants follow simple binary-algebraic rules.
All four types share the same special cases:
* Any complete array of `1` bits (Exclusive of the Sign Bit) represents Infinity: `INF`
* Infinity will inherit the sign
* Any complete array of `0` bits represents 0
* Any complete array of `0` bits with a negative Sign Bit represents `NaN`

Certums are fixed-points with small whole number components. 
Quartas are fixed-points where a fourth of the bits are whole numbers, the rest are fractional.
Dimids are fixed-points with half whole number bits, half fractional bits.
Acutes are fixed-points that lack whole number bits, instead representing a 0-1 fraction with the highest precision possible.

Certums and Acutes are quite simple and defined by simple structures:

Unsigned Certum:
* Given `n` bits of data:
* `log_2(n)` whole bits, `n - log_2(n) + 1` fractional bits.

Signed Certums:
* Given `n` bits of data:
* `1` sign bit, `log_2(n) - 1` whole bits, `n - log_2(n)` fractional bits.

Unsigned Acutes:
* Given `n` bits of data:
* `n` fractional bits.

Signed Acutes:
* Given `n` bits of data:
* `1` sign bit, `n - 1` fractional bits.

Ex: 

8-Bit
* Signed Certum: `01010101` = 2.65625
* Signed -Certum: `11010101` = -2.65625
* Unsigned Certum: `10010101` = 4.65625
* Signed Acute: `01010101` = 0.6640625
* Signed -Acute: `11010101` = -0.6640625
* Unsigned Acute: `10101011` = 0.66796875

For data ranges, paste any value into wolframalpha to see its whole format.

Certums Data Ranges:
| Bit Depth | Min Signed             | Max Signed            | Min Unsigned | Max Unsigned           |
| ---       | ---                    | ---                   | ---          | ---                    |
| 8 Bits    | -3 - 2<sup>-5</sup>    | 3 + 2<sup>-5</sup>    | 0            | 7 + 2<sup>-5</sup>     |
| 16 Bits   | -7 - 2<sup>-12</sup>   | 7 + 2<sup>-12</sup>   | 0            | 15 + 2<sup>-12</sup>   |
| 32 Bits   | -15 - 2<sup>-27</sup>  | 15 + 2<sup>-27</sup>  | 0            | 31 + 2<sup>-27</sup>   |
| 64 Bits   | -31 - 2<sup>-58</sup>  | 31 + 2<sup>-58</sup>  | 0            | 63 + 2<sup>-58</sup>   |
| 128 Bits  | -63 - 2<sup>-121</sup> | 63 + 2<sup>-121</sup> | 0            | 127 + 2<sup>-121</sup> |