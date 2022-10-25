use diesel::MysqlConnection;

pub trait InquiryRepository {}

pub struct InquirySqlRepository {
    _connection: MysqlConnection,
}

impl InquiryRepository for InquirySqlRepository {}

impl InquirySqlRepository {
    pub fn new(connection: MysqlConnection) -> Self {
        InquirySqlRepository {
            _connection: connection,
        }
    }
}
