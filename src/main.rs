use std::error::Error;
use std::time::Duration;

use chrono::Utc;

use rppal::gpio;
use structopt::StructOpt;

pub mod sensor;

const GPIO_TESTWIRE: u8 = 26;

#[derive(Debug, structopt::StructOpt)]
pub struct Opt {
    #[structopt(short, long)]
    pub address: String,
    #[structopt(short, long)]
    pub topic: String,
    #[structopt(short, long, default_value = "10")]
    pub interval: u64,
    #[structopt(short, long)]
    pub username: String,
    #[structopt(short, long)]
    pub password: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    loop {
        let mut gpio = gpio::Gpio::new()?;

        let mut device = sensor::Dht11Device::new(GPIO_TESTWIRE);

        let reading = match device.read(&mut gpio) {
            Ok(v) => v,
            Err(e) => {
                dbg!(e);
                std::thread::sleep(Duration::from_secs(3));
                continue;
            }
        };

        let (temp, humidity) = reading;

        println!("[{}] Temp: {}C Humidity: {}%", Utc::now(), temp, humidity);

        std::thread::sleep(Duration::from_secs(opt.interval));
    }
}

pub struct InnerState {
    device: sensor::Dht11Device,
}
