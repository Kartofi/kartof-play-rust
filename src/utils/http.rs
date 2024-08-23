pub fn get_(url: &str) -> Option<String> {
    match reqwest::blocking::get(url) {
        Ok(req) => match req.text() {
            Ok(text) => Some(text),
            Err(_err) => {
                println!("ERROR REQ URL: {}", url);
                None
            }
        },
        Err(_err) => {
            println!("ERROR REQ URL: {}", url);
            None
        }
    }
}

use std::io::BufRead;
use std::sync::{Arc, Mutex, Once};
use std::thread;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::header::USER_AGENT;
use reqwest::Proxy;

use crate::{HTTP_FREQUENCY_TIMEOUT, HTTP_REQUEST_TIMEOUT};

struct RequestRateLimiter {
    last_request: Instant,
}

static mut RATE_LIMITER: Option<Arc<Mutex<RequestRateLimiter>>> = None;
static INIT: Once = Once::new();

fn init_rate_limiter() {
    INIT.call_once(|| {
        let limiter = RequestRateLimiter {
            last_request: Instant::now() - HTTP_FREQUENCY_TIMEOUT,
        };
        unsafe {
            RATE_LIMITER = Some(Arc::new(Mutex::new(limiter)));
        }
    });

    let limiter = unsafe { RATE_LIMITER.as_ref().unwrap().clone() };
    let mut limiter_guard = limiter.lock().unwrap();

    let now = Instant::now();
    let elapsed = now.duration_since(limiter_guard.last_request);

    if elapsed < HTTP_FREQUENCY_TIMEOUT {
        let sleep_duration = HTTP_FREQUENCY_TIMEOUT - elapsed;
        thread::sleep(sleep_duration);
    }

    limiter_guard.last_request = Instant::now();
}

pub fn get(url: &str) -> Option<String> {
    if url.contains("animeschedule.net") && url != "https://animeschedule.net/" {
        init_rate_limiter();
    }
    // List of common User-Agent strings
    let user_agents = vec![
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 15_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Mobile/15E148 Safari/604.1",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7; rv:91.0) Gecko/20100101 Firefox/91.0",
        // Add more user agents as needed
    ];

    // Load proxies from file
    let proxies = match load_proxies_from_file() {
        Ok(proxies) => proxies,
        Err(err) => {
            println!("Failed to load proxies: {}", err);
            return None;
        }
    };

    // Choose a random user agent
    let user_agent = user_agents.choose(&mut thread_rng()).unwrap();

    // Choose a random proxy
    let proxy = proxies.choose(&mut thread_rng()).unwrap();

    // Create a client with the selected user agent and proxy
    let client = match Proxy::http(proxy) {
        Ok(proxy) => reqwest::blocking::Client::builder()
            .proxy(proxy)
            .timeout(HTTP_REQUEST_TIMEOUT)
            .build()
            .unwrap(),
        Err(_) => reqwest::blocking::Client::new(),
    };

    match client
        .get(url)
        .header(USER_AGENT, user_agent.to_string())
        .send()
    {
        Ok(req) => match req.text() {
            Ok(text) => Some(text),
            Err(_err) => {
                println!("ERROR REQ URL: {}", url);
                None
            }
        },
        Err(_err) => {
            println!("ERROR REQ URL: {}", url);
            None
        }
    }
}

fn load_proxies_from_file() -> std::io::Result<Vec<String>> {
    let file = std::fs::File::open("proxies.txt")?;
    let reader = std::io::BufReader::new(file);
    let proxies: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    Ok(proxies)
}
