#[derive(Debug, Clone)]
struct Item {
    price: Price,
}

#[derive(Debug, Clone)]
enum Currency {
    JPY,
}

#[derive(Debug, Clone)]
pub struct Price {
    currency: Currency,
    amount: usize,
}
