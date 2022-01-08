use std::sync::Arc;

use crate::model::{advertisement::Advertisement, error::AppError, search::SearchCondition};

pub type ArcIAdvertisementRepository = Arc<dyn IAdvertisementRepository + Sync + Send>;

pub trait IAdvertisementRepository {
    fn search(&self, cond: SearchCondition) -> Result<Vec<Advertisement>, AppError>;
}
