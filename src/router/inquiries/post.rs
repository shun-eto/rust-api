use crate::{
    modules::services_module::ServicesModule,
    service::inquiry::post_inquiry_service::PostInquiryService,
};

pub struct PostInquiryRouter {
    _post_inquiry_service: PostInquiryService,
}

impl PostInquiryRouter {
    pub fn new(services_module: ServicesModule) -> Self {
        Self {
            _post_inquiry_service: services_module.post_inquiry_service,
        }
    }

    pub async fn handle(&self) {}
}
