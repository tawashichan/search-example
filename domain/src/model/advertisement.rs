use super::item::Price;

#[derive(Debug, Clone)]
pub struct Advertisement {
    name: AdvertisementName,
    price: Price,
    tags: Vec<AdvertisementTag>,
}

#[derive(Debug, Clone)]
struct AdvertisementName(String);

#[derive(Debug, Clone)]
struct AdvertisementTag(String);
