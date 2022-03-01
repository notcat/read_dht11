use core::fmt::Debug;
use std::time::{Duration, Instant};
use thiserror::Error as ThisError;

use rppal::gpio::{self, Gpio, PullUpDown};

#[derive(Debug, ThisError)]
pub enum Dht11Error {
    #[error("device gpio error")]
    GpioError(#[from] gpio::Error),
    #[error("read failure")]
    ReadFail,
    #[error("read timeout")]
    ReadTimeout,
    #[error("checksum failure")]
    InvalidChecksum,
}

pub struct Dht11Device {
    pin: u8,
}

impl Dht11Device {
    pub fn new(pin: u8) -> Self {
        Self { pin }
    }
    /// Reads the DHT11 sensor and returns the temperature and humidity.
    pub fn read(&mut self, gpio: &mut Gpio) -> Result<(f32, f32), Dht11Error> {
        // allocate space for data processing
        let pulse_buf = [Instant::now(); 86];

        {
            // request readout from sensor
            let mut iopin = gpio.get(self.pin)?.into_io(gpio::Mode::Input);
            iopin.set_reset_on_drop(false);
            iopin.set_pullupdown(PullUpDown::PullUp);

            let future = Instant::now() + Duration::from_millis(1);
            while Instant::now() < future {}

            iopin.set_mode(gpio::Mode::Output);
            iopin.set_low();
        }

        let pin = gpio.get(self.pin)?;

        // now wait 20ms
        let future = Instant::now() + Duration::from_millis(18);
        while Instant::now() < future {}

        // configure as input pin and read the pulses
        let pin = pin.into_input_pullup();

        let pulse_buf = Self::read_pulses(pin, pulse_buf)?;

        let buf = Self::parse_pulses(pulse_buf)?;

        if (buf[0..4].iter().map(|x| *x as u32).sum::<u32>() & 0xFF) as u8 != buf[4] {
            return Err(Dht11Error::InvalidChecksum);
        }

        let temp = buf[2] as f32;
        let humidity = buf[0] as f32;

        Ok((temp, humidity))
    }

    /// Reads the pulses from the GPIO bus as fast as we can.
    fn read_pulses(
        pin: gpio::InputPin,
        mut buf: [Instant; 86],
    ) -> Result<[Instant; 86], Dht11Error> {
        let now = Instant::now();
        let deadline = Duration::from_secs(1);
        let mut state: gpio::Level = gpio::Level::Low;
        let mut i = 0;
        while i < 84 {
            if now.elapsed() > deadline {
                return Err(Dht11Error::ReadTimeout);
            }
            let current_state = pin.read();
            if state != current_state {
                state = current_state;
                buf[i] = Instant::now();
                i += 1;
            }
        }
        Ok(buf)
    }

    /// Parses the pulse times into bits, and then into bytes.
    #[inline(always)]
    fn parse_pulses(pulses: [Instant; 86]) -> Result<[u8; 5], Dht11Error> {
        let mut buf: [u8; 5] = [0u8; 5]; // hi hi damon
        for (c, i) in (2..42).enumerate() {
            let k = i * 2;
            let k2 = k + 1;
            let diff = (pulses[k2] - pulses[k]).as_micros();
            let bit = if diff > 51 { 1 } else { 0 };
            buf[c as usize / 8] |= bit << (7 - (c as usize % 8));
        }
        Ok(buf)
    }
}
