use core::fmt::Debug;
use std::error::Error;
use std::slice::SliceIndex;
use std::time::{Duration, Instant};
use std::{thread, time};
// pub mod foo;

use rppal::gpio::{self, Gpio, PullUpDown, Trigger};

const GPIO_TESTWIRE: u8 = 26;

fn main() -> Result<(), Box<dyn Error>> {
    // let pin = Gpio::new()?.get(GPIO_TESTWIRE).unwrap();

    let mut gpio = Gpio::new()?;

    let mut device = Dht11Device::new(GPIO_TESTWIRE);

    dbg!(device.read(&mut gpio));

    Ok(())
}

pub struct Dht11Device {
    pin: u8,
}

pub enum Bit {
    Zero,
    One,
}

impl Dht11Device {
    pub fn new(pin: u8) -> Self {
        Self { pin }
    }

    fn wait_for_high(&mut self) {
        // while self.pin.is_low() {}
    }

    fn wait_for_low(&mut self, gpio: &mut Gpio) -> Result<gpio::Level, Box<dyn Error>> {
        let mut iopin = gpio.get(self.pin).unwrap().into_input();

        let res = iopin.poll_interrupt(false, Some(Duration::from_millis(100)));
        match res {
            Ok(Some(level)) => Ok(level),
            Ok(None) => todo!("idk when this happens or why"),
            Err(error) => Err(error.into()),
        }
    }

    /// Reads the DHT11 sensor and returns the temperature and humidity.
    pub fn read(&mut self, gpio: &mut Gpio) {
        // allocate space for data processing
        let mut data = [0u16, 80];

        dbg!("sex 😳😳😳😳");

        // setup wiring and send reset signal
        {
            let mut iopin = gpio.get(self.pin).unwrap().into_io(gpio::Mode::Input);
            iopin.set_reset_on_drop(false);
            iopin.set_pullupdown(PullUpDown::PullUp);

            iopin.set_mode(gpio::Mode::Output);

            iopin.set_low();
        }

        let now = Instant::now();
        let sleep_dur = time::Duration::from_micros(20);

        let mut pin = gpio.get(self.pin).unwrap().into_input_pullup();
        pin.set_interrupt(Trigger::Both).unwrap();

        thread::sleep(sleep_dur - now.elapsed());

        let now = Instant::now();

        let first_trigger = pin.poll_interrupt(false, None);

        dbg!("first: ", first_trigger, now.elapsed());

        let second_trigger = pin.poll_interrupt(false, None);

        dbg!("second: ", second_trigger, now.elapsed());

        // read 40 bits of data
        for i in 0..40 {
            let k = i * 2;
            let k2 = k * 2 + 1;
            if !matches!(
                pin.poll_interrupt(false, Some(Duration::from_millis(1))),
                Ok(Some(gpio::Level::Low))
            ) {
                panic!("expected low {}ms", now.elapsed().as_millis());
            }
            data[k] = now.elapsed().as_micros() as u16;

            if !matches!(
                pin.poll_interrupt(false, Some(Duration::from_millis(1))),
                Ok(Some(gpio::Level::High))
            ) {
                panic!("expected high {}ms", now.elapsed().as_millis());
            }
            data[k2] = now.elapsed().as_micros() as u16;
        }

        dbg!("sex complete", data);
    }

    /// Reads a bit pulse returning the result.
    pub fn read_bit_pulse(&mut self) -> Option<Bit> {
        None
    }
}

// raspberry pi

// START PULSE
// pulldown for 18 us
// pullup and wait for response

// dht11

// RESPONSE SIGNAL
// pulldown for 54 us
// pullup for 80us

// Data being sent

// data consists of a pulldown for 54 us, and then a pullup for 24us or 70us,
// depending on whether it is a 0 or a 1
