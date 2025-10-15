# Certums

This is a simple numeric extension in rust that implements low-level fixed-sized fixed-point numbers
as well as extensions and utilities for them in many different forms.

Certums and their variants are fixed-point values with whole number and fractional components.
Acutes are fractional numbers with no whole number components.

The goal is to have a flexible and high-precision number system for tasks that rely largely on fractional components over supporting whole-number values.

These values are easily (Though lossily) convertable between eachother through simple bit-shifts, making them a convenient and easy-to-use format.

### Changelog

- V0.0.2 - Fix algebra and add 128-bit certums

* **NOTE: Due to the lack of widespread support, conversion of Certums (and variants) to f16s and f128s is not supported**

### Currently Implemented Types and Operands

* 8-128 bit Certums, 8-128 bit Unsigned Certums
* Casting / Float conversion for all types
* Negation for all types
* Addition and Subtraction for all types
* MIN/MAX/MINF/MAXF constants for all types
* PI and E constants for some types

### More information:

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
| ---       | ---                    | ---                   | ---          | ---                    | ---   | ---   |
| 8 Bits    | -2 + 2<sup>-6</sup>    | 2 - 2<sup>-6</sup>    | 0            | 4 - 2<sup>-6</sup>     | 1:6   | 2:6   |
| 16 Bits   | -4 + 2<sup>-13</sup>   | 4 - 2<sup>-13</sup>   | 0            | 8 - 2<sup>-13</sup>    | 2:13  | 3:13  |
| 32 Bits   | -8 + 2<sup>-28</sup>   | 8 - 2<sup>-28</sup>   | 0            | 16 - 2<sup>-28</sup>   | 3:28  | 4:28  |
| 64 Bits   | -16 + 2<sup>-59</sup>  | 16 - 2<sup>-59</sup>  | 0            | 32 - 2<sup>-59</sup>   | 4:59  | 5:59  |
| 128 Bits  | -32 + 2<sup>-122</sup> | 32 - 2<sup>-122</sup> | 0            | 64 - 2<sup>-122</sup>  | 5:122 | 6:122 |

Quarta Data Ranges:
| Bit Depth | Min Signed             | Max Signed            | Min Unsigned | Max Unsigned          | Integer:Fraction Signed | Integer:Fraction Unsigned |
| ---       | ---                    | ---                   | ---          | ---                   | ---  | ---  |
| 8 Bits    | -2 + 2<sup>-6</sup>    | 2 - 2<sup>-6</sup>    | 0            | 4 - 2<sup>-6</sup>    | 1:6  | 2:6  |
| 16 Bits   | -8 + 2<sup>-12</sup>   | 8 - 2<sup>-12</sup>   | 0            | 16 - 2<sup>-12</sup>  | 3:12 | 4:12 |
| 32 Bits   | -128 + 2<sup>-24</sup> | 128 - 2<sup>-24</sup> | 0            | 256 - 2<sup>-24</sup> | 7:24 | 8:24 |
| 64 Bits   | -2<sup>15</sup> + 2<sup>-48</sup> | 2<sup>15</sup> - 2<sup>-48</sup> | 0 | 2<sup>16</sup> - 2<sup>-48</sup> | 15:48 | 16:48 |
| 128 Bits  | -2<sup>31</sup> + 2<sup>-96</sup> | 2<sup>31</sup> - 2<sup>-96</sup> | 0 | 2<sup>32</sup> - 2<sup>-96</sup> | 31:96 | 32:96 |

Dimid Data Ranges:
| Bit Depth | Min Signed            | Max Signed           | Min Unsigned | Max Unsigned         | Integer:Fraction Signed | Integer:Fraction Unsigned |
| ---       | ---                   | ---                  | ---          | ---                  | ---  | ---  |
| 8 Bits    | -8 + 2<sup>-4</sup>   | 8 - 2<sup>-4</sup>   | 0            | 16 - 2<sup>-4</sup>  | 3:4  | 4:4  |
| 16 Bits   | -128 + 2<sup>-8</sup> | 128 - 2<sup>-8</sup> | 0            | 256 - 2<sup>-8</sup> | 7:8  | 8:8  |
| 32 Bits   | -2<sup>15</sup> + 2<sup>-16</sup> | 2<sup>15</sup> - 2<sup>-16</sup> | 0 | 2<sup>16</sup> - 2<sup>-16</sup> | 15:16 | 16:16 |
| 64 Bits   | -2<sup>31</sup> + 2<sup>-32</sup> | 2<sup>31</sup> - 2<sup>-32</sup> | 0 | 2<sup>32</sup> - 2<sup>-32</sup> | 31:32 | 32:32 |
| 128 Bits  | -2<sup>63</sup> + 2<sup>-64</sup> | 2<sup>63</sup> - 2<sup>-64</sup> | 0 | 2<sup>64</sup> - 2<sup>-64</sup> | 63:64 | 64:64 |

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