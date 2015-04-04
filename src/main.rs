
extern crate hyper;
extern crate rustc_serialize;

extern crate getopts;
use getopts::Options;
use std::env;
mod header;
mod mashape;
use std::io::Read;
use hyper::Client;
use rustc_serialize::json;
use std::thread;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use mashape::CurrencyResponse;
use std::str::FromStr;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn fetch () -> f32 {
    let key_str: &'static str = env!("MASHAP_KEY");
    let mut client = Client::new();

    let mut res = client.get("https://currencyconverter.p.mashape.com?from=USD&to=BRL&from_amount=1")
    .header(Connection(vec![ConnectionOption::Close]))
    .header(header::XMashapeKey {key: key_str.to_string()})
    .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let c: CurrencyResponse = json::decode(&body).unwrap();
    c.to_amount
}


fn main () {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("d", "", "Delta between cycles.", "DELTA");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let delta = match matches.opt_str("d") {
        Some(v) => { f32::from_str(v.as_ref()).unwrap() }
        None => {
            println!("Will use default value for delta: 10 cents");
            10.0
        }
    };

    //let delta = f32::from_str(deltaOpt.unwrap().as_ref()).unwrap();
    // let mut args = std::env::args();
    let mut last_collected = 0.0;
    // args.next();
    // let delta = f32::from_str(args.next().unwrap().as_ref()).unwrap();


    loop {

        let amount = fetch();
        let diff = last_collected - amount;
        if diff.abs() >= delta {
            println!("Ooops! Huge variation {}, before: {}, now: {}", diff, last_collected, amount);

        }
        last_collected = amount;

        thread::sleep_ms(3000)
    }

}
