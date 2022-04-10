mod a11y;
mod args;
mod logging;
use a11y::{BusProxy, StatusProxy};
use atspi::registry::RegistryProxy;
use color_eyre::eyre::{
    Result,
    WrapErr,
};
use std::str::FromStr;
use tracing::{
    debug,
    //error,
    info,
};
use zbus::{
    dbus_proxy, export::futures_util::StreamExt, Address, Connection, ConnectionBuilder,
    };
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    logging::init();
    let _args = args::parse();
    let connection = Connection::session().await.wrap_err("unable to connect to dbus session")?;
    let bproxy = BusProxy::new(&connection).await.wrap_err("error while creating a proxy to the session buss entrypoint to the atspi registrid")?;
    let sproxy = StatusProxy::new(&connection).await.wrap_err("unable to create status proxy to the dbus interface used to get and set screen reader status")?;
    let addr = bproxy.get_address().await.wrap_err("error while getting the address of the atspi registry on the system bus")?;
    let rstream = Address::from_str(&addr).wrap_err("can't convert the address into a dbus stream")?;
    let rconnection = ConnectionBuilder::address(rstream)?.build().await.wrap_err("error while creating a connection to the dbus registry")?;
    let rproxy = RegistryProxy::new(&rconnection).await.wrap_err("unable to create atspi registry proxy from connection")?;
    // NOTE: this doesn't work for some reason? Always shows false for me (Tait)

    sproxy.set_screen_reader_enabled(true).await.wrap_err("error setting the screen_reader_enabled property on the status dbus interface")?;
    sproxy.set_is_enabled(true).await.wrap_err("error setting the screen_reader_enabled property on the status dbus interface")?;

    info!("Hello, world!");
    debug!("Found the a11y bus {}!", addr);
    let _screen_reader_enabled = {
        if sproxy.screen_reader_enabled().await? {
            debug!(
                "screen reader state has been successfully set to on in the accessibility system"
            );
            true
        } else {
            debug!("failed to set screen reader property to on in the accessibility system");
            false
        }
    };
    rproxy
        .register_event("object:state-changed:focused")
        .await?;
    rproxy.register_event("object:text-caret-moved").await?;

    let events = rproxy.get_registered_events().await?;
    tracing::debug!("Events registerd: {}", events.len());
    for (e1, e2) in events {
        tracing::debug!("Event {},{} is registered", e1, e2);
    }
    let mut stream = bproxy.receive_all_signals().await?;
    while let Some(_e) = stream.next().await {
        debug!("Event received!");
    }
    Ok(())
}
