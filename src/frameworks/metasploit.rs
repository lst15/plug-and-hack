use metasploit::client::Client;

struct MetasploitClient {
    client:Client,
}

impl MetasploitClient {
    pub async fn new(host:&str, port:i32, username:&str, password:&str) -> Result<MetasploitClient, ()> {
        let client=Client::new(host,port,username,password,false);
        // let sessions = sessions::list::<Value>(client).expect("ERR");

        Ok(Self{client})
    }
}