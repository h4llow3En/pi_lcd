extern crate pi_lcd;

#[cfg(test)]
mod tests {
    use pi_lcd::*;

    #[test]
    #[should_panic(expected = "UnsupportedHardware")]
    fn create_lcd(){
        let _lcd = HD44780::new(11,10,[6,5,4,1],20,4);
    }
}
