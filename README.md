# pi_lcd

pi_lcd is a Rust Crate to connect a HD44780 LCD to your Raspberry Pi.  
[![Build Status](https://travis-ci.org/h4llow3En/pi_lcd.svg?branch=master)](https://travis-ci.org/h4llow3En/pi_lcd)


## Pinout

#### Raspberry Pi with 26 Pins (A, B)
![pin26](https://cloud.githubusercontent.com/assets/6068259/15632138/ff3d1de2-2588-11e6-9064-504d2a9d3277.png)

#### Raspberry Pi with 26 Pins (A+, B+, Pi2 B)
![pin40](https://cloud.githubusercontent.com/assets/6068259/15666295/55cfc91a-2710-11e6-9c7c-4ba529680d9b.png)

## Example

```rust
use extern pi_lcd;

// create a new lcd
let lcd = HD44780::new(11,10,[6,5,4,1],20,4);

// send a String to the lcd at row 0
lcd.send_string("Hello World".to_string(),0);
```


## Usage & Documentation

Coming soon...

## License
This work is licensed under the _MIT_ license. See `LICENSE` for more information.
