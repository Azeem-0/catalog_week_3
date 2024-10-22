use actix_web::Error;

pub struct MongoDB {
    name: String,
}

impl MongoDB {
    pub async fn init() -> Result<Self, Error> {
        Ok(MongoDB {
            name: String::from("Azeem"),
        })
    }
}
