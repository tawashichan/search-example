use crate::interfaces::advertisement_repository::ArcIAdvertisementRepository;

#[derive(Clone)]
pub struct SearchService {
    advertisement_repository: ArcIAdvertisementRepository,
}

impl SearchService {
    pub fn new(advertisement_repository: ArcIAdvertisementRepository) -> Self {
        SearchService {
            advertisement_repository,
        }
    }
}
