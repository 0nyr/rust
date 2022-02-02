pub struct Client {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

impl Client {

    // constructor
    pub fn new(
        id: i32,
        name: String, 
        email: String, 
        password: String
    ) -> Client {
        Client {
            id,
            name,
            email,
            password
        }
    }
}

// to string method
impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, 
            "Client {{ \n
            id: {}, \n
            name: {}, \n
            email: {}, \n
            password: {}, \n
            }}", self.id, self.name, self.email, 
            self.password
        )
    }
}