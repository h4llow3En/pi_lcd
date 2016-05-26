#![allow(deprecated)]
extern crate cylus;

use cylus::Cylus;
use std::time::Duration;

static CGRAM_ADDRESS: u8 = 0x40;
static RS_COMMAND: bool = false;
static RS_DATA: bool = true;

pub struct HD44780 {
    rs: Cylus,
    e: Cylus,
    data: [Cylus; 4],
    cols: u32,
    rows: u32,
    lines: Vec<u8>
}

impl HD44780 {
    pub fn new(disp_rs: u32, disp_e: u32, datalines: [u32;4], disp_cols: u32, disp_rows: u32) -> HD44780 {
        let rs = Cylus::new(disp_rs);
        let e = Cylus::new(disp_e);
        let data: [Cylus; 4] = [Cylus::new(datalines[0]),Cylus::new(datalines[1]),Cylus::new(datalines[2]),Cylus::new(datalines[3])];
        let lines: Vec<u8>;


        match disp_rows {
            1 => lines = vec![0x80],
            2 => lines = vec![0x80, 0xC0],
            3 => lines = vec![0x80, 0xC0, 0x94],
            4 => lines = vec![0x80, 0xC0, 0x94, 0xD4],
            _ => lines = vec![0x80]
        };

        let result = HD44780 {
            rs: rs,
            e: e,
            data: data,
            cols: disp_cols,
            rows: disp_rows,
            lines: lines
        };



        result
    }

    pub fn init(&self) {
        self.command(0x33);
        self.command(0x32);
        self.command(0x28);
        self.command(0x0C);
        self.command(0x06);
        self.clean();
    }
    pub fn clean(&self) {
        //clean display
        HD44780::command(self, 0x01);
    }

    pub fn command(&self, bits: u8) {
        //send command
        HD44780::send_byte(self, bits, RS_COMMAND);
    }

    pub fn send_string(&self, text: String, row: u32) {
        //prepare String to write it to the LCD
        let mut message: Vec<u8> = Vec::new();
        let col = self.cols;
        let row = row % self.rows;

        //TODO: implement check for custom characters

        for i in text.chars(){
            message.push(i as u8);
        }

        message.truncate(col as usize);

        self.select_row(row);
        self.write(message);
    }

    pub fn create_char(&self, address: u8, bitmap: [u8;8]) {
        //TODO: Check for address in range 0..8
        //send new custom character to cgram address

        self.command(CGRAM_ADDRESS | address << 3);
        for row in 0..bitmap.len(){
            self.send_byte(bitmap[row as usize], RS_DATA);
        }
    }

    fn e_wait(){
        std::thread::sleep(Duration::new(0, 50));
    }

    fn select_row(&self, row: u32) {
        //select the row where the String should be printed at
        HD44780::send_byte(self, self.lines[row as usize], RS_COMMAND);
    }

    fn write(&self, charlist: Vec<u8>) {
        //send every single char to send_byte
        for x in charlist {
            HD44780::send_byte(self, x, RS_DATA);
        }
    }

    fn send_byte(&self, bits: u8, mode: bool) {
        match mode {
            true  => self.rs.high(),
            false => self.rs.low()
        }
        self.data[0].low();
        self.data[1].low();
        self.data[2].low();
        self.data[3].low();
        if bits & 0x10 == 0x10{
            self.data[0].high();
        }
        if bits & 0x20 == 0x20{
            self.data[1].high();
        }
        if bits & 0x40 == 0x40{
            self.data[2].high();
        }
        if bits & 0x80 == 0x80{
            self.data[3].high();
        }
        HD44780::e_wait();
        self.e.high();
        HD44780::e_wait();
        self.e.low();
        self.data[0].low();
        self.data[1].low();
        self.data[2].low();
        self.data[3].low();
        if bits & 0x01 == 0x01{
            self.data[0].high();
        }
        if bits & 0x02 == 0x02{
            self.data[1].high();
        }
        if bits & 0x04 == 0x04{
            self.data[2].high();
        }
        if bits & 0x08 == 0x08{
            self.data[3].high();
        }
        HD44780::e_wait();
        self.e.high();
        HD44780::e_wait();
        self.e.low();
    }
}
