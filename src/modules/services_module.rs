use crate::service::inquiry::post_inquiry_service::PostInquiryService;

use super::repositories_module::RepositoriesModule;

pub struct ServicesModule {
    pub post_inquiry_service: PostInquiryService,
}

impl ServicesModule {
    pub fn new(repositories_module: RepositoriesModule) -> Self {
        Self {
            post_inquiry_service: PostInquiryService::new(repositories_module.inquiry_repository),
        }
    }
}
