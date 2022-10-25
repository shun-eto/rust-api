use diesel::MysqlConnection;

use crate::repositories::inquiry_repository::InquirySqlRepository;

pub struct RepositoriesModule {
    pub inquiry_repository: InquirySqlRepository,
}

impl RepositoriesModule {
    pub fn new(connection: MysqlConnection) -> Self {
        Self {
            inquiry_repository: InquirySqlRepository::new(connection),
        }
    }
}
