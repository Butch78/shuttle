use tokio::runtime::Runtime;

use crate::{Factory, ResourceBuilder};

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

use serde::Serialize;
use serde::de::DeserializeOwned;
use bincode::{serialize_into, deserialize_from};

use std::error::Error;

use async_trait::async_trait;

pub struct Persist;


impl Persist {
    pub fn save<T: Serialize>(key: &str, struc: T) -> Result<(), Box<dyn Error>> {

        fs::create_dir_all("shuttle_data")?;
        // Create walk up the directory with a Test. 
        let filename = format!("/shuttle_data/{}.bin", key);
        let file = File::create(format!("{}", filename)).unwrap();
        let mut writer = BufWriter::new(file);
        Ok(serialize_into(&mut writer, &struc)?)
    }

    pub fn load<T>(key: &str) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let filename = format!("data/{}.bin", key);
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let data = deserialize_from(reader)?;
        Ok(data)
    }

}   


#[async_trait]
impl ResourceBuilder<Persist> for Persist {
    fn new() -> Self {
        Self {} 
    }

    async fn build(
        self,
        _factory: &mut dyn Factory, 
        _runtime: &Runtime, 
    ) -> Result<Persist, crate::Error> {

        Ok(Persist {})

        
    }
}


