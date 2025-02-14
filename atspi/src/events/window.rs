use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.a11y.atspi.Event.Window")]
trait Window {
    /// Activate signal
    #[dbus_proxy(signal)]
    fn activate(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Close signal
    #[dbus_proxy(signal)]
    fn close(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Create signal
    #[dbus_proxy(signal)]
    fn create(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Deactivate signal
    #[dbus_proxy(signal)]
    fn deactivate(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// DesktopCreate signal
    #[dbus_proxy(signal)]
    fn desktop_create(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// DesktopDestroy signal
    #[dbus_proxy(signal)]
    fn desktop_destroy(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Destroy signal
    #[dbus_proxy(signal)]
    fn destroy(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Lower signal
    #[dbus_proxy(signal)]
    fn lower(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Maximize signal
    #[dbus_proxy(signal)]
    fn maximize(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Minimize signal
    #[dbus_proxy(signal)]
    fn minimize(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Move signal
    #[dbus_proxy(signal)]
    fn move_(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// PropertyChange signal
    #[dbus_proxy(signal)]
    fn property_change(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Raise signal
    #[dbus_proxy(signal)]
    fn raise(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Reparent signal
    #[dbus_proxy(signal)]
    fn reparent(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Resize signal
    #[dbus_proxy(signal)]
    fn resize(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Restore signal
    #[dbus_proxy(signal)]
    fn restore(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Restyle signal
    #[dbus_proxy(signal)]
    fn restyle(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// Shade signal
    #[dbus_proxy(signal)]
    fn shade(&self, event: super::Event<'_>) -> zbus::Result<()>;

    /// uUshade signal
    #[dbus_proxy(signal)]
    fn u_ushade(&self, event: super::Event<'_>) -> zbus::Result<()>;
}
