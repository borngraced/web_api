use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{Responder},
    Request, Response,
};

#[derive(Debug)]
pub struct ErrorT {
    pub status: Status,
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ErrorT {
    fn respond_to(self, _: &'r Request<'_>) -> Result<rocket::Response<'static>, Status> {
        let content = format!("Bad: {}", self.message);
        Response::build()
            .sized_body(content.len(), Cursor::new(content))
            .header(ContentType::new("application", "x-person"))
            .status(self.status)
            .ok()
    }
}
