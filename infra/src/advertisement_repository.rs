use domain::{
    interfaces::advertisement_repository::IAdvertisementRepository,
    model::{advertisement::Advertisement, error::AppError, search::SearchCondition},
};

pub struct AdvertisementRepository {}

impl IAdvertisementRepository for AdvertisementRepository {
    fn search(&self, cond: SearchCondition) -> Result<Vec<Advertisement>, AppError> {
        Ok(vec![])
    }
}

impl AdvertisementRepository {
    pub fn new() -> Self {
        AdvertisementRepository {}
    }
}
