mod args;
mod logging;
use std::{
  str::FromStr,
};
use tracing::{
  info,
  debug,
  //error,
};
use zbus::{export::futures_util::StreamExt, Address, Connection, ConnectionBuilder, dbus_proxy, Result};
use a11y;
#[tokio::main]
async fn main() -> Result<(), Box<Std::error::error> {
    logging::init();
    let _args = args::parse();
    let connection = Connection::session().await?;
    let bproxy = BusProxy::new(&connection).await?;
    let sproxy = StatusProxy::new(&connection).await?;
    let addr = bproxy.get_address().await?;
    let rstream = Address::from_str(&addr)?;
    let rconnection = ConnectionBuilder::address(rstream)?.build().await?;
    let rproxy = RegistryProxy::new(&rconnection).await?;
    // NOTE: this doesn't work for some reason? Always shows false for me (Tait)
    let screen_reader_enabled = sproxy.screen_reader_enabled().await?;
    sproxy.set_screen_reader_enabled(true).await?;
    sproxy.set_is_enabled(true).await?;
    //sproxy.set_screen_reader_enabled(false).await?;
    info!("Hello, world!");
    info!("Found the a11y bus {}!", addr);
    info!("Screen ready is active? {}", screen_reader_enabled);
    rproxy.register_event("object:state-changed:focused").await?;
    rproxy.register_event("object:text-caret-moved").await?;
    //rproxy.register_event("object:text-caret-movedd").await?;
    let events = rproxy.get_registered_events().await?;
    tracing::debug!("Events registerd: {}", events.len());
    for (e1, e2) in events {
      tracing::debug!("Event {},{} is registered", e1, e2);
    }
    let mut stream = bproxy.receive_all_signals().await?;
    while let Some(_e) = stream.next().await {
      println!("EVENT!");
      tracing::info!("\tEvent received!");
    }
    Ok(())
}
/// # DBus interface proxies for: `org.a11y.Status`, `org.a11y.Bus`
///
/// This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
/// Source: `Interface '/org/a11y/bus' from service 'org.a11y.Bus' on session bus`.
///
/// You may prefer to adapt it, instead of using it verbatim.
///
/// More information can be found in the
/// [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
/// section of the zbus documentation.
///
/// This DBus object implements
/// [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
/// (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
///
/// * [`zbus::fdo::PropertiesProxy`]
/// * [`zbus::fdo::IntrospectableProxy`]
/// * [`zbus::fdo::PeerProxy`]
///
/// …consequently `zbus-xmlgen` did not generate code for the above interfaces.

#[dbus_proxy(
  interface = "org.a11y.Status",
  default_service = "org.a11y.Bus",
  default_path = "/org/a11y/bus"
)]
trait Status {
    /// IsEnabled property
    #[dbus_proxy(property)]
    fn is_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_is_enabled(&self, value: bool) -> zbus::Result<()>;

    /// ScreenReaderEnabled property
    #[dbus_proxy(property)]
    fn screen_reader_enabled(&self) -> zbus::Result<bool>;
    #[dbus_proxy(property)]
    fn set_screen_reader_enabled(value: bool, &self) -> zbus::Result<()>;
}

#[dbus_proxy(
  interface = "org.a11y.Bus",
  default_path = "/org/a11y/bus"
)]
trait Bus {
    /// GetAddress method
    fn get_address(&self) -> zbus::Result<String>;
}
/// # DBus interface proxy for: `org.a11y.atspi.Registry`
///
/// This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
/// Source: `Interface '/org/a11y/atspi/registry' from service 'org.a11y.atspi.Registry'`.
///
/// You may prefer to adapt it, instead of using it verbatim.
///
/// More information can be found in the
/// [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
/// section of the zbus documentation.
///

#[dbus_proxy(
  interface = "org.a11y.atspi.Registry",
  default_service = "org.a11y.atspi.Registry",
  default_path = "/org/a11y/atspi/registry"
)]
trait Registry {
    /// DeregisterEvent method
    fn deregister_event(&self, event: &str) -> zbus::Result<()>;

    /// GetRegisteredEvents method
    fn get_registered_events(&self) -> zbus::Result<Vec<(String, String)>>;

    /// RegisterEvent method
    fn register_event(&self, event: &str) -> zbus::Result<()>;

    /// EventListenerDeregistered signal
    #[dbus_proxy(signal)]
    fn event_listener_deregistered(&self, bus: &str, path: &str) -> zbus::Result<()>;

    /// EventListenerRegistered signal
    #[dbus_proxy(signal)]
    fn event_listener_registered(&self, bus: &str, path: &str) -> zbus::Result<()>;
}
