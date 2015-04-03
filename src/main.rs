extern crate hyper;
extern crate rustc_serialize;

mod header;
mod mashape;
use std::io::Read;
use hyper::Client;
use rustc_serialize::json;
use std::thread;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use mashape::CurrencyResponse;

let key_str: &'static str = env!("MASHAP_KEY");

fn fetch () -> f32 {
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


    let mut last_collected = 0.0;

    loop {

        let amount = fetch();
        let diff = last_collected - amount;
        if diff.abs() > 10.0 {
            println!("Ooops! Huge variation {}, before: {}, now: {}", diff, last_collected, amount);

        }
        last_collected = amount;

        thread::sleep_ms(3000)
    }

}
