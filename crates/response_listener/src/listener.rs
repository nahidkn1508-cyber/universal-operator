pub struct ResponseListener;

impl ResponseListener {
    pub fn new() -> Self {
        Self
    }

    pub fn receive<'a>(&self, response: &'a str) -> &'a str {
        response
    }
}
