use adapter::{Adapter, AdapterErrors};
use anyhow::{bail, Ok, Result};
use bluer::{self, Session};

pub(crate) mod adapter;
pub(crate) mod device;
pub(crate) mod device_list;
mod utils;

// Representation of client system
pub struct Client {
    adapter_names: Vec<String>,
    session: Session,
    adapter: Adapter,
}

impl Client {
    pub async fn new() -> Result<Client> {
        let session = bluer::Session::new().await?;
        let mut adapter_names = session.adapter_names().await?;
        adapter_names.sort();
        let default_adapter = if adapter_names.iter().any(|name| name == "hci0") {
            session.adapter("hci0").unwrap()
        } else {
            match adapter_names.first() {
                Some(adapter_name) => session.adapter(adapter_name).unwrap(),
                None => bail!(AdapterErrors::AdapterNotFound),
            }
        };

        Ok(Self {
            session,
            adapter_names,
            adapter: Adapter::new(default_adapter).await?,
        })
    }

    pub fn adapter(&self) -> &Adapter {
        &self.adapter
    }

    pub fn adapter_mut(&mut self) -> &mut Adapter {
        &mut self.adapter
    }

    pub fn adapter_names(&self) -> &Vec<String> {
        &self.adapter_names
    }
}

#[cfg(test)]
mod test {
    use anyhow::{Ok, Result};
    use tokio::{pin, sync::Mutex};

    use super::Client;

    #[tokio::test]
    async fn list_adapters() -> Result<()> {
        let client = Client::new().await?;
        println!("{:?}", client.adapter_names);
        println!("{:?}", client.adapter.adapter_info().await);
        Ok(())
    }

    #[tokio::test]
    async fn discovered_devices() -> Result<()> {
        let client = Client::new().await?;
        println!("{:?}", client.adapter().known_devices().await);
        Ok(())
    }

    #[tokio::test]
    async fn discover_devices() -> Result<()> {
        let mut client = Client::new().await?;
        client.adapter_mut().discover_devices(10).await?;
        Ok(())
    }
}
