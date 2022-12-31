pub mod database {
    use std::{
        fs::{OpenOptions, self, File}, 
        io::{BufWriter, BufReader, BufRead, Write}
    };

    use tokio_postgres::{Error, NoTls};

    use crate::errors::error_handler::error_handler as error;
    
    pub fn read_database_url() -> Result<String, error::FileError> {
        let file = std::fs::File::open("config/database.yml")?;
        let url: serde_yaml::Mapping = serde_yaml::from_reader(file)?;
    
        Ok(url["database_url"].as_str().unwrap().to_string())
    }

    pub async fn set_database_url(
        user_name: String, 
        pw: String, 
        host: String,
        port: String,
        mut url: String
    ) -> Result<(), error::FileError> {
        fs::remove_file("config/database.yml").ok();
    
        let f = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open("config/database.yml")
            .expect("Couldn't open file");
    
        if url == "" {
            url = format!("database_url: postgresql://{}:{}@{}:{}/", user_name, pw, host, port);
        } else {
            url = format!("database_url: {}", url);
        }
    
        let mut f = BufWriter::new(f);
        write!(f, "{}", url)?;
    
        Ok(())
    }

    pub async fn setup_database() -> Result<(), Error> {
        println!("Please note that you need to have Postgres installed!");
            
        let mut host = rpassword::prompt_password("Enter your host. If it's localhost, leave blank and press Enter").unwrap();
        let user_name = rpassword::prompt_password("Enter your postgres user name").unwrap();
        let pw = rpassword::prompt_password("Enter your postgres password").unwrap();
        let mut port = rpassword::prompt_password("Enter port. Default is 5433. Leave blank if default and press Enter").unwrap();
    
        host = if host == "" { "localhost".to_string() } else { host };
        port = if port == "" { "5433".to_string() } else { port };
    
        match set_database_url(
            user_name.clone(), 
            pw.clone(), 
            host.clone(), 
            port.clone(), 
            "".to_string()
        ).await {
            Ok(_) => {
                let config = format!("host={} user={} password={} port={}", host, user_name, pw, port);
                let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;
                
                tokio::spawn(async {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
    
                println!("Creating database...");
                
                let file = File::open("config/schema.sql").unwrap();
                let reader = BufReader::new(file);
    
                for line in reader.lines().enumerate() {
                    match client.batch_execute(line.1.unwrap().as_str()).await {
                        Err(e) => println!("error: {}", e),
                        _ => (),
                    }
                }
    
                Ok(())
            },
            Err(e) => Ok(println!("{}", e))
        }
    }
}