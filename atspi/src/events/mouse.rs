use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Mouse")]
trait Mouse {
    /// Abs signal
    #[dbus_proxy(signal)]
    fn abs(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Button signal
    #[dbus_proxy(signal)]
    fn button(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Rel signal
    #[dbus_proxy(signal)]
    fn rel(&self, event: super::Event<'_>) -> zbus::Result<()>;
}
