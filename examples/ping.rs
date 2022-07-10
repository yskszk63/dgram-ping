use std::env;
use std::time::Instant;

use tokio::time::{interval, timeout, Duration};

use dgram_ping::Pinger;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let ip = if let Some(ip) = env::args().nth(1) {
        ip
    } else {
        anyhow::bail!("usage: %prog [ip]");
    };

    let duration = Duration::from_secs(1);
    let mut interval = interval(duration);
    let mut pinger = Pinger::open(ip.parse()?)?;

    loop {
        interval.tick().await;

        let begin = Instant::now();
        if let Ok(result) = timeout(duration, pinger.ping()).await {
            result?;
        } else {
            println!("timeout.");
            continue;
        }
        println!(
            "{}: {} ms",
            ip,
            Instant::now().duration_since(begin).as_millis()
        );
    }
}
