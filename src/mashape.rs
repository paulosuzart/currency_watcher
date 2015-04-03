#[derive(RustcDecodable, RustcEncodable)]
pub struct CurrencyResponse  {
    pub from: String,
    pub to: String,
    pub to_amount: f32,
}
