# Certums

This is a simple numeric extension in rust that implements low-level fixed-sized fixed-point numbers
as well as extensions and utilities for them in many different forms.

Certums and their variants are fixed-point values with whole number and fractional components.
Acutes are fractional numbers with no whole number components.

The goal is to have a flexible and high-precision number system for tasks that rely largely on fractional components over supporting whole-number values.

These values are easily (Though losslessly) convertable between eachother through simple bit-shifts, making them a convenient and easy-to-use format.

* **NOTE: Due to the lack of widespread support, conversion of Certums (and variants) to f16s and f128s is not supported**

* **Additionally: Only Certums and Float conversion are currently implemented**


Certums (and variants) and Acutes are quite simple and defined by simple structures, `n` representing bit count:

Unsigned Certum:
* `log_2(n) - 1` whole bits, `n - log_2(n) + 1` fractional bits.
Signed Certums:
* `1` sign bit, `log_2(n) - 2` whole bits, `n - log_2(n) + 1` fractional bits.

Unsigned Quarta:
* `n / 4` whole bits, `n - n / 4` fractional bits.
Signed Quarta:
* `1` sign bit, `n / 4 - 1` whole bits, `n - n / 4` fractional bits.

Unsigned Dimid:
* `n / 2` whole bits, `n - n / 2` fractional bits.
Signed Dimid:
* `1` sign bit, `n / 2 - 1` whole bits, `n - n / 2` fractional bits.

Unsigned Acutes:
* `n` fractional bits.
Signed Acutes:
* `1` sign bit, `n - 1` fractional bits.

For data ranges, type any value into wolframalpha to see its full decimal format.

Certum Data Ranges:
| Bit Depth | Min Signed             | Max Signed            | Min Unsigned | Max Unsigned           | Integer:Fraction Signed | Integer:Fraction Unsigned |
| ---       | ---                    | ---                   | ---          | ---                    | --- | --- |
| 8 Bits    | -2 + 2<sup>-6</sup>    | 2 - 2<sup>-6</sup>    | 0            | 4 - 2<sup>-6</sup>     | 1:6 ||
| 16 Bits   | -4 + 2<sup>-13</sup>   | 4 - 2<sup>-13</sup>   | 0            | 8 - 2<sup>-13</sup>    | ||
| 32 Bits   | -8 + 2<sup>-28</sup>   | 8 - 2<sup>-28</sup>   | 0            | 16 - 2<sup>-28</sup>   |||
| 64 Bits   | -16 + 2<sup>-59</sup>  | 16 - 2<sup>-59</sup>  | 0            | 32 - 2<sup>-59</sup>   |||
| 128 Bits  | -32 + 2<sup>-122</sup> | 32 - 2<sup>-122</sup> | 0            | 64 - 2<sup>-122</sup>  |||

Quarta Data Ranges:
| Bit Depth | Min Signed             | Max Signed            | Min Unsigned | Max Unsigned          |
| ---       | ---                    | ---                   | ---          | ---                   |
| 8 Bits    | -2 + 2<sup>-6</sup>    | 2 - 2<sup>-6</sup>    | 0            | 4 - 2<sup>-6</sup>    |
| 16 Bits   | -8 + 2<sup>-12</sup>   | 8 - 2<sup>-12</sup>   | 0            | 16 - 2<sup>-12</sup>  |
| 32 Bits   | -128 + 2<sup>-24</sup> | 128 - 2<sup>-24</sup> | 0            | 256 - 2<sup>-24</sup> |
| 64 Bits   | -2<sup>15</sup> + 2<sup>-48</sup> | 2<sup>15</sup> - 2<sup>-48</sup> | 0 | 2<sup>16</sup> - 2<sup>-48</sup> |
| 128 Bits  | -2<sup>31</sup> + 2<sup>-96</sup> | 2<sup>31</sup> - 2<sup>-96</sup> | 0 | 2<sup>32</sup> - 2<sup>-96</sup> |

Dimid Data Ranges:
| Bit Depth | Min Signed            | Max Signed           | Min Unsigned | Max Unsigned         |
| ---       | ---                   | ---                  | ---          | ---                  |
| 8 Bits    | -8 + 2<sup>-4</sup>   | 8 - 2<sup>-4</sup>   | 0            | 16 - 2<sup>-4</sup>  |
| 16 Bits   | -128 + 2<sup>-8</sup> | 128 - 2<sup>-8</sup> | 0            | 256 - 2<sup>-8</sup> |
| 32 Bits   | -2<sup>15</sup> + 2<sup>-16</sup> | 2<sup>15</sup> - 2<sup>-16</sup> | 0 | 2<sup>16</sup> - 2<sup>-16</sup> |
| 64 Bits   | -2<sup>31</sup> + 2<sup>-32</sup> | 2<sup>31</sup> - 2<sup>-32</sup> | 0 | 2<sup>32</sup> - 2<sup>-32</sup> |
| 128 Bits  | -2<sup>63</sup> + 2<sup>-64</sup> | 2<sup>63</sup> - 2<sup>-64</sup> | 0 | 2<sup>64</sup> - 2<sup>-64</sup> |

Acute Data Ranges:
| Bit Depth | Min Signed            | Max Signed           | Min Unsigned | Max Unsigned         |
| ---       | ---                   | ---                  | ---          | ---                  |
| 8 Bits    | -1 + 2<sup>-7</sup>   | 1 - 2<sup>-7</sup>   | 0            | 1 - 2<sup>-8</sup>   |
| 16 Bits   | -1 + 2<sup>-15</sup>  | 1 - 2<sup>-15</sup>  | 0            | 1 - 2<sup>-16</sup>  |
| 32 Bits   | -1 + 2<sup>-31</sup>  | 1 - 2<sup>-31</sup>  | 0            | 1 - 2<sup>-32</sup>  |
| 64 Bits   | -1 + 2<sup>-63</sup>  | 1 - 2<sup>-63</sup>  | 0            | 1 - 2<sup>-64</sup>  |
| 128 Bits  | -1 + 2<sup>-127</sup> | 1 - 2<sup>-127</sup> | 0            | 1 - 2<sup>-128</sup> |

Translated:<br>
Certum - Fixed<br>
Quarta - Quarter<br>
Dimidium - Half<br>
Acute - Sharp<br>