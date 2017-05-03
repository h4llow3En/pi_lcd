//! A Rust crate to connect a HD44780 lcd display
//!
//! # Example
//! ```no_run
//! use pi_lcd::*;
//!
//! // create a new lcd
//! let lcd = HD44780::new(11,10,[6,5,4,1],20,4);
//!
//! // send a String to the lcd at row 0
//! lcd.send_string("Hello World".to_string(),0);
//! ```

extern crate cupi;
extern crate regex;

use cupi::{CuPi, PinOutput, DigitalWrite};
use std::time::Duration;
use std::cell::RefCell;
use regex::Regex;


static CGRAM_ADDRESS: u8 = 0x40;
static COMMAND: bool = false;
static DATA: bool = true;

/// The display handle
pub struct HD44780 {
    rs: RefCell<PinOutput>,
    e: RefCell<PinOutput>,
    data: Vec<RefCell<PinOutput>>,
    cols: u32,
    rows: u32,
    lines: Vec<u8>,
}

impl HD44780 {
    /// Creates a new HD44780 instance with `disp_rs` as rs pin, `disp_e` as enabled pin, `datalines` as data4 to data7
    ///
    /// `disp_cols` are the number of columns
    /// `disp_rows` are the number of rows
    pub fn new(disp_rs: u32,
               disp_e: u32,
               datalines: [u32; 4],
               disp_cols: u32,
               disp_rows: u32)
               -> HD44780 {
        let raspi = CuPi::new().unwrap();
        let rs = RefCell::new(raspi.pin(disp_rs as usize).unwrap().output());
        let e = RefCell::new(raspi.pin(disp_e as usize).unwrap().output());
        let mut data: Vec<RefCell<PinOutput>> = Vec::new();
        for x in 0..4 {
            data.push(RefCell::new(raspi.pin(datalines[x] as usize).unwrap().output()));
        }

        let lines: Vec<u8>;


        match disp_rows {
            1 => lines = vec![0x80],
            2 => lines = vec![0x80, 0xC0],
            3 => lines = vec![0x80, 0xC0, 0x94],
            4 => lines = vec![0x80, 0xC0, 0x94, 0xD4],
            _ => lines = vec![0x80],
        };

        let result = HD44780 {
            rs: rs,
            e: e,
            data: data,
            cols: disp_cols,
            rows: disp_rows,
            lines: lines,
        };



        result
    }

    /// Initializes the display and clears it
    pub fn init(&self) {
        self.command(0x33);
        self.command(0x32);
        self.command(0x28);
        self.command(0x0C);
        self.command(0x06);
        self.clear();
    }

    /// Clears the display
    pub fn clear(&self) {
        self.command(0x01);
    }

    /// Sends a given byte as a command
    pub fn command(&self, bits: u8) {
        self.send_byte(bits, COMMAND);
    }

    /// Parses a String and and outputs it to the given row
    pub fn send_string(&self, text: String, row: u32) {
        let re_char: Regex = Regex::new(r"^\\cg:([0-7])").unwrap();
        let mut message: Vec<u8> = Vec::new();
        let col = self.cols;
        let row = row % self.rows;

        // TODO: implement check for custom characters

        for i in text.chars() {
            message.push(i as u8);
        }

        message.truncate(col as usize);

        self.select_row(row);
        self.write(message);
    }

    /// Creates a new custom character from a bitmap on the given `address`
    pub fn create_char(&self, address: u8, bitmap: [u8; 8]) -> Result<u8, &'static str> {
        // send new custom character to cgram address
        match address {
            0...7 => {
                self.command(CGRAM_ADDRESS | address << 3);
                for row in &bitmap {
                    self.send_byte(bitmap[*row as usize], DATA);
                }
                Ok(address)
            },
            _ => Err("address must be between 0 and 7"),
        }

    }

    fn select_row(&self, row: u32) {
        // select the row where the String should be printed at
        self.send_byte(self.lines[row as usize], COMMAND);
    }

    fn write(&self, charlist: Vec<u8>) {
        // send every single char to send_byte
        for x in charlist {
            self.send_byte(x, DATA);
        }
    }

    fn send_byte(&self, bits: u8, mode: bool) {
        if mode {
            self.rs.borrow_mut().high().unwrap();
        } else {
            self.rs.borrow_mut().low().unwrap();
        }
        self.data[0].borrow_mut().low().unwrap();
        self.data[1].borrow_mut().low().unwrap();
        self.data[2].borrow_mut().low().unwrap();
        self.data[3].borrow_mut().low().unwrap();
        if bits & 0x10 == 0x10 {
            self.data[0].borrow_mut().high().unwrap();
        }
        if bits & 0x20 == 0x20 {
            self.data[1].borrow_mut().high().unwrap();
        }
        if bits & 0x40 == 0x40 {
            self.data[2].borrow_mut().high().unwrap();
        }
        if bits & 0x80 == 0x80 {
            self.data[3].borrow_mut().high().unwrap();
        }
        e_wait();
        self.e.borrow_mut().high().unwrap();
        e_wait();
        self.e.borrow_mut().low().unwrap();
        self.data[0].borrow_mut().low().unwrap();
        self.data[1].borrow_mut().low().unwrap();
        self.data[2].borrow_mut().low().unwrap();
        self.data[3].borrow_mut().low().unwrap();
        if bits & 0x01 == 0x01 {
            self.data[0].borrow_mut().high().unwrap();
        }
        if bits & 0x02 == 0x02 {
            self.data[1].borrow_mut().high().unwrap();
        }
        if bits & 0x04 == 0x04 {
            self.data[2].borrow_mut().high().unwrap();
        }
        if bits & 0x08 == 0x08 {
            self.data[3].borrow_mut().high().unwrap();
        }
        e_wait();
        self.e.borrow_mut().high().unwrap();
        e_wait();
        self.e.borrow_mut().low().unwrap();
    }
}

/// Waits 50 ns to let the display recognize the enabled pin
pub fn e_wait() {
    std::thread::sleep(Duration::new(0, 50));
}
