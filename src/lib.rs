#![allow(deprecated)]
extern crate cupi;

use cupi::*;
use std::time::Duration;

static CGRAM_ADDRESS: u8 = 0x40;
static RS_COMMAND: bool = false;
static RS_DATA: bool = true;

pub struct HD44780 {
    disp_rs: u32,
    disp_e: u32,
    datalines: [u32; 4],
    raspi: CuPi,
    cols: u32,
    rows: u32,
    lines: Vec<u8>,
}

impl HD44780 {
    pub fn new(disp_rs: u32,
               disp_e: u32,
               datalines: [u32; 4],
               disp_cols: u32,
               disp_rows: u32)
               -> HD44780 {
        let raspi = CuPi::new().unwrap();

        let lines: Vec<u8>;


        match disp_rows {
            1 => lines = vec![0x80],
            2 => lines = vec![0x80, 0xC0],
            3 => lines = vec![0x80, 0xC0, 0x94],
            4 => lines = vec![0x80, 0xC0, 0x94, 0xD4],
            _ => lines = vec![0x80],
        };

        let result = HD44780 {
            disp_rs: disp_rs,
            disp_e: disp_e,
            datalines: datalines,
            raspi: raspi,
            cols: disp_cols,
            rows: disp_rows,
            lines: lines,
        };



        result
    }

    pub fn init(&self) {
        self.command(0x33);
        self.command(0x32);
        self.command(0x28);
        self.command(0x0C);
        self.command(0x06);
        self.clear();
    }
    pub fn clear(&self) {
        // clear display
        self.command(0x01);
    }

    pub fn command(&self, bits: u8) {
        // send command
        self.send_byte(bits, RS_COMMAND);
    }

    pub fn send_string(&self, text: String, row: u32) {
        // prepare String to write it to the LCD
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

    pub fn create_char(&self, address: u8, bitmap: [u8; 8]) {
        // TODO: Check for address in range 0..8
        // send new custom character to cgram address

        self.command(CGRAM_ADDRESS | address << 3);
        for row in 0..bitmap.len() {
            self.send_byte(bitmap[row as usize], RS_DATA);
        }
    }

    fn e_wait() {
        std::thread::sleep(Duration::new(0, 50));
    }

    fn select_row(&self, row: u32) {
        // select the row where the String should be printed at
        self.send_byte(self.lines[row as usize], RS_COMMAND);
    }

    fn write(&self, charlist: Vec<u8>) {
        // send every single char to send_byte
        for x in charlist {
            self.send_byte(x, RS_DATA);
        }
    }

    fn send_byte(&self, bits: u8, mode: bool) {
        let mut rs = self.raspi.pin(self.disp_rs as usize).unwrap().output();
        let mut e = self.raspi.pin(self.disp_e as usize).unwrap().output();
        let mut data: Vec<PinOutput> = Vec::new();
        for x in 0..4 {
            data.push(self.raspi.pin(self.datalines[x] as usize).unwrap().output());
        }

        if mode {
            rs.high().unwrap();
        } else {
            rs.low().unwrap();
        }
        data[0].low().unwrap();
        data[1].low().unwrap();
        data[2].low().unwrap();
        data[3].low().unwrap();
        if bits & 0x10 == 0x10 {
            data[0].high().unwrap();
        }
        if bits & 0x20 == 0x20 {
            data[1].high().unwrap();
        }
        if bits & 0x40 == 0x40 {
            data[2].high().unwrap();
        }
        if bits & 0x80 == 0x80 {
            data[3].high().unwrap();
        }
        HD44780::e_wait();
        e.high().unwrap();
        HD44780::e_wait();
        e.low().unwrap();
        data[0].low().unwrap();
        data[1].low().unwrap();
        data[2].low().unwrap();
        data[3].low().unwrap();
        if bits & 0x01 == 0x01 {
            data[0].high().unwrap();
        }
        if bits & 0x02 == 0x02 {
            data[1].high().unwrap();
        }
        if bits & 0x04 == 0x04 {
            data[2].high().unwrap();
        }
        if bits & 0x08 == 0x08 {
            data[3].high().unwrap();
        }
        HD44780::e_wait();
        e.high().unwrap();
        HD44780::e_wait();
        e.low().unwrap();
    }
}
