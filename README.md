


# CPU
The Gameboy's CPU is a custom chip -- SHarp LR35902.
The CPU is similar to the more popular Intel 8080 and the Zilog Z80.
This is useful to know as the 8080 and Z80 were popular chips for devices back in the 70s and 80s.

## CPU Registers
Registers are responsible for holding on to little pieces of data that the CPU can manipulate when it executes various instructions.
Since the LR35902 is an 8-bit CPU, which means that each register can hold 8 bits of data, the CPU has 8 registers,
"a", "b", "c", "d", "e", "f", "h", "l".

```rust
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
    l: u8,
}
```

While the CPU only has 8 bit registers, there are instructions that allow the game to read and write 16 bits at the same time.
Therefore, we'll need the ability to read and write these "virtual" 16 bit registers.
These registers are referred to as "af" ("a" and "f" combined), "bc", "de", and "hl" (following the same rule).

```rust
struct Registers { a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, h: u8, l: u8, }
impl Registers {
  fn get_bc(&self) -> u16 {
    (self.b as u16) << 8 | (self.c as u16)
  }

  fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0xFF) as u8;
  }
}

```

For reading the "bc" register, we first treat the "b" register as a ```u16```
(this effectively just adds a byte of all 0s to the most significant position of the number 
(the most significant position being the left-most position)).
We then shift the "b" register 8 positions to the left so that it's occupying the most significant position.
Finally, we bitwise OR the "c" register with the "b" register.
The result is a two byte number with the contents of "b" in the most significant position 
and the contents of "c" in the least significant byte position.

Let's discuss the usage of ```& 0xFF00``` and ```& 0xFF```.

```0xFF00``` represents the hexadecimal number ```1111 1111 0000 0000```.

When using the ```&``` operator, which is a bitwise AND, with ```0xFF00``` with a value, 
we are masking the lower 8 bits of that value.

```
0xFF00 = 1111 1111 0000 0000
value  = 1010 1010 1010 1010 (example)
result = 1010 1010 0000 0000 (when using &)
```

When this is then combined with ```>> 8```, we are shifting the bits 8 positions to the right.
This gives us the following result:
```0000 0000 1010 1010```. We then cast this to a ```u8``` to get ```1010 1010```.

## Flags Register
The "f" register is a special register called the "flags" register.
The lower four bits of the register are **always** 0s 
and the CPU automatically writes to the upper four bits when certain things happen.
In other words, the CPU "flags" certain states.
They have the following names and positions:
- Bit 7: "zero"
- Bit 6: "subtract"
- Bit 5: "half carry"
- Bit 4: "carry"

Diagram of the flags register:
```
    ┌-> Carry
 ┌-+> Subtraction
 | |
1111 0000
| |
└-+> Zero
  └-> Half Carry
```

## LR35902 Instructions