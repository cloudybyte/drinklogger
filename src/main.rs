extern crate pwr_hd44780;

use std::{env, io::Read, thread, time::Duration};

//use mfrc522::Mfrc522;
use pwr_hd44780::Hd44780;
use rfid_rs::{MFRC522, picc};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};

use std::io;


fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    if let Some(option) = args.get(1) {
        if option == &"start".to_string() {
            let spi = create_spi().unwrap();
            let mut reader = rfid_rs::MFRC522 { spi };
            reader.init().expect("Init failed!");
            test(reader);
        }
    } else {
        panic!("either 'start' or 'write' as first argument required");
    }
    
    /*let spi = create_spi().unwrap();
    let mut reader = rfid_rs::MFRC522 { spi };
    reader.init().expect("Init failed!");

    test(reader);*/
}


fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}



fn test(mut mfrc522: MFRC522) {

    loop {
        let new_card = mfrc522.new_card_present().is_ok();

        if new_card {
            let key: rfid_rs::MifareKey = [0xffu8; 6];

            let uid = match mfrc522.read_card_serial() {
                Ok(u) => u,
                Err(e) => {
                    println!("Could not read card: {:?}", e);
                    continue
                },
            };

            let mut block = 4;
            let len = 18;

            match mfrc522.authenticate(picc::Command::MfAuthKeyA, block, key, &uid) {
                Ok(_) => println!("Authenticated card"),
                Err(e) => {
                    println!("Could not authenticate card {:?}", e);
                    continue
                }
            }
            match mfrc522.mifare_read(block, len) {
                Ok(response) => println!("Read block {}: {:?}", block, response.data),
                Err(e) => {
                    println!("Failed reading block {}: {:?}", block, e);
                    continue
                }
            }

            block = 1;

            match mfrc522.authenticate(picc::Command::MfAuthKeyA, block, key, &uid) {
                Ok(_) => println!("Authenticated card"),
                Err(e) => {
                    println!("Could not authenticate card {:?}", e);
                    continue
                }
            }
            match mfrc522.mifare_read(block, len) {
                Ok(response) => println!("Read block {}: {:?}", block, response.data),
                Err(e) => {
                    println!("Failed reading block {}: {:?}", block, e);
                    continue
                }
            }

            mfrc522.halt_a().expect("Could not halt");
            mfrc522.stop_crypto1().expect("Could not stop crypto1");
        }
    }

}


fn print_lcd() {
        // create the LCD's bus instance;
    // use device at address 0x27 on the first I2C bus
    let lcd_bus = pwr_hd44780::I2CBus::new(
        "/dev/i2c-1", 0x27,
    ).unwrap();

    // create the direct LCD's instance;
    // use bus created before and assume LCD's width x height = 20 x 4
    let mut lcd = pwr_hd44780::DirectLcd::new(
        Box::new(lcd_bus),
        16, 2,
    ).unwrap();

    // finally - print our text
    lcd.clear().unwrap();
    thread::sleep(Duration::from_secs(3));
    lcd.print("Hello World! :-)").unwrap();
    thread::sleep(Duration::from_secs(3));
    lcd.clear().unwrap();
    lcd.set_backlight(false).unwrap();
}





/* fn test() {
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .max_speed_hz(1_000_000)
        .mode(hal::spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).unwrap();

    let pin = Pin::new(25);
    pin.export().unwrap();
    while !pin.is_exported() {}
    pin.set_direction(Direction::Out).unwrap();
    pin.set_value(1).unwrap();

    let mut mfrc522 = Mfrc522::new(spi, pin).unwrap();

    let vers = mfrc522.version().unwrap();

    println!("VERSION: 0x{:x}", vers);

    assert!(vers == 0x91 || vers == 0x92);

    loop {
        const CARD_UID: [u8; 4] = [34, 246, 178, 171];
        const TAG_UID: [u8; 4] = [128, 170, 179, 76];

        if let Ok(atqa) = mfrc522.reqa() {
            if let Ok(uid) = mfrc522.select(&atqa) {
                println!("UID: {:?}", uid);
                println!("uid: {}", uid.bytes());

                if uid.bytes() == &CARD_UID {
                    led.off();
                    println!("CARD");
                } else if uid.bytes() == &TAG_UID {
                    led.on();
                    println!("TAG");
                }
            }
        }
    }
} */