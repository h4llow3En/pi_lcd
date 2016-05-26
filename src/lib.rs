#![allow(deprecated)]
#[allow(dead_code)]
extern crate cylus;

use cylus::Cylus;
use std::time::Duration;

static cgram_address: u8 = 0x40;
static rs_command: bool = false;
static rs_data: bool = true;

pub struct HD44780 {
    rs: Cylus,
    e: Cylus,
    data: [Cylus; 4],
    cols: u32,
    rows: u32,
    lines: Box<[u8]>
}

impl HD44780 {
    pub fn new(disp_rs: u32, disp_e: u32, datalines: [u32;4], disp_cols: u32, disp_rows: u32) -> HD44780 {
        let rs = Cylus::new(disp_rs);
        let e = Cylus::new(disp_e);
        let data: [Cylus; 4] = [Cylus::new(datalines[0]),Cylus::new(datalines[1]),Cylus::new(datalines[2]),Cylus::new(datalines[3])];
        let lines: Box<[u8]>;

        match disp_rows {
            1 => lines = Box::new([0x80]),
            2 => lines = Box::new([0x80, 0xC0]),
            3 => lines = Box::new([0x80, 0xC0, 0x94]),
            4 => lines = Box::new([0x80, 0xC0, 0x94, 0xD4]),
            _ => lines = Box::new([0x80])
        };

        let result = HD44780 {
            rs: rs,
            e: e,
            data: data,
            cols: disp_cols,
            rows: disp_rows,
            lines: lines
        };

        HD44780::command(0x33);
        HD44780::command(0x32);
        HD44780::command(0x28);
        HD44780::command(0x0C);
        HD44780::command(0x06);
        HD44780::clean();

        result
    }

    pub fn clean() {
        //clean display
        unimplemented!()
    }

    pub fn command(bits: u8) {
        //send command
        unimplemented!()
    }

    pub fn send_string(text: String, row: u32) {
        //prepare String to write it to the LCD
        unimplemented!()
    }
    
    fn e_wait(){
        std::thread::sleep(Duration::new(0, 50));
    }

    fn create_char(address: u8, bitmap: [u8;8]) {
        //send new custom character to cgram address
        unimplemented!()
    }

    fn select_row(row: u32) {
        //select the row where the String should be printed at
        unimplemented!()
    }

    fn write() {
        //send every single char to send_byte
        unimplemented!()
    }

    fn send_byte(bits: u8, mode: bool) {
        unimplemented!()
    }
}
