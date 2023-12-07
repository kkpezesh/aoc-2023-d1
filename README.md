# Advent of Code 2023 Day 1

Documenting my [Advent of Code 2023](https://adventofcode.com/2023) solutions as I ramp up on [Rust](https://doc.rust-lang.org/book/)

## Testing

```sh
cargo run
```

## Problem Statement

Global snow production is impacted and must be restored by December 25th
* Alternative solutions, like a weather machine, are insufficient to meet global demand

Before you're launched into the sky to troubleshoot snow production issues, the Elves need you to calibrate their trebuchet
* The calibration document has been erroneously amended by a less experienced Elf

### Part 1

Each line of the original calibration document contained a single calibration value.
To restore the calibration document to its original state, combine the first digit and last digit on a line to form a single two-digit number (from 0-99)

Once you've restored the state of the document, calculate the sum of all calibration values and submit your solution

#### Example

```
1337 => 17
qwerty2bac6wards => 26
ra3cecar => 33

sum => 76
```

### Part 2

Your calculation from part 1 is insufficient. The calibration document may also contain digits 0-9 spelled out (like `four` or `seven`)

#### Example

```
seventeen28sixteen43fourty => 74
onehors3andcarr1age => 11
ten3footm5gicbeanst8lk => 38

sum => 123
```