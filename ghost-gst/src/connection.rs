use rand::Rng;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct Connection<
    const BASE_DELAY_MS: u64,
    const MAX_DELAY_MS: u64,
    const FACTOR: u32,
    const JITTER: u32,
> {
    stream: TcpStream,
}

struct BackoffStrategy<
    const BASE_DELAY_MS: u64,
    const MAX_DELAY_MS: u64,
    const FACTOR: u32,
    const JITTER: u32,
> {
    base_delay: Duration,
    max_delay: Duration,
    factor: f64,
    jitter: f64,
}

impl<const BASE_DELAY_MS: u64, const MAX_DELAY_MS: u64, const FACTOR: u32, const JITTER: u32>
    BackoffStrategy<BASE_DELAY_MS, MAX_DELAY_MS, FACTOR, JITTER>
{
    const fn new() -> Self {
        Self {
            base_delay: Duration::from_millis(BASE_DELAY_MS),
            max_delay: Duration::from_millis(MAX_DELAY_MS),
            factor: FACTOR as f64 / 100.0,
            jitter: JITTER as f64 / 100.0,
        }
    }

    fn next_backoff(&self, attempt: u32) -> Duration {
        let mut delay = self.base_delay.mul_f64(self.factor.powi(attempt as i32));
        if delay > self.max_delay {
            delay = self.max_delay;
        }
        if self.jitter > 0.0 {
            let mut rng = rand::rng();
            let jitter = rng.random_range(-self.jitter..=self.jitter);
            delay = delay.mul_f64(1.0 + jitter);
        }
        delay
    }
}

impl<const BASE_DELAY_MS: u64, const MAX_DELAY_MS: u64, const FACTOR: u32, const JITTER: u32>
    Connection<BASE_DELAY_MS, MAX_DELAY_MS, FACTOR, JITTER>
{
    pub fn new(host: &str, port: u16, max_attempts: u32) -> Result<Self, Error> {
        let backoff = BackoffStrategy::<BASE_DELAY_MS, MAX_DELAY_MS, FACTOR, JITTER>::new();

        let mut last_error = None;
        for attempt in 0..max_attempts {
            match TcpStream::connect((host, port)) {
                Ok(stream) => return Ok(Connection { stream }),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_attempts - 1 {
                        let delay = backoff.next_backoff(attempt);
                        std::thread::sleep(delay);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| Error::other("Max attempts reached")))
    }

    pub fn send(&mut self, message: &str) -> Result<(), Error> {
        self.stream.write_all(message.as_bytes())?;
        Ok(())
    }

    pub fn receive(&mut self) -> Result<String, Error> {
        let mut buffer = [0; 1024];
        let bytes_read = self.stream.read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }
}
