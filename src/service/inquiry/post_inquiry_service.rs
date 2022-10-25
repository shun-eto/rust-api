use crate::repositories::inquiry_repository::InquirySqlRepository;

pub struct PostInquiryService {
    _inquiry_repository: InquirySqlRepository,
}

impl PostInquiryService {
    pub fn new(inquiry_repository: InquirySqlRepository) -> Self {
        Self {
            _inquiry_repository: inquiry_repository,
        }
    }

    pub fn execute(&self) {}
}

pub fn post_inquiry_service() {}
