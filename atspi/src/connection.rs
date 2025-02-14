use std::ops::Deref;

use zbus::Address;

use crate::{bus::BusProxy, registry::RegistryProxy};

/// A connection to the at-spi bus
pub struct Connection {
    registry: RegistryProxy<'static>,
}

impl Connection {
    /// Open a new connection to the bus
    #[tracing::instrument]
    pub async fn open() -> zbus::Result<Self> {
        // Grab the a11y bus address from the session bus
        let a11y_bus_addr = {
            tracing::debug!("Connecting to session bus");
            let session_bus = zbus::Connection::session().await?;
            tracing::debug!(
                name = session_bus.unique_name().map(|n| n.as_str()),
                "Connected to session bus"
            );
            let proxy = BusProxy::new(&session_bus).await?;
            tracing::debug!("Getting a11y bus address from session bus");
            proxy.get_address().await?
        };
        tracing::debug!(address = %a11y_bus_addr, "Got a11y bus address");
        let addr: Address = a11y_bus_addr.parse()?;
        Self::connect(addr).await
    }

    pub async fn connect(bus_addr: Address) -> zbus::Result<Self> {
        tracing::debug!("Connecting to a11y bus");
        let bus = zbus::ConnectionBuilder::address(bus_addr)?.build().await?;
        tracing::debug!(
            name = bus.unique_name().map(|n| n.as_str()),
            "Connected to a11y bus"
        );
        // The Proxy holds a strong reference to a Connection, so we only need to store the proxy
        let registry = RegistryProxy::new(&bus).await?;

        Ok(Self { registry })
    }
}

impl Deref for Connection {
    type Target = RegistryProxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.registry
    }
}
