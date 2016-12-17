use philipshue::bridge;

#[cfg(all(feature = "nupnp", not(feature = "upnp")))]
pub fn discover() -> Vec<String> {
    use philipshue::hue::Discovery;
    bridge::discover().unwrap().into_iter().map(Discovery::into_ip).collect()
}

#[cfg(feature = "upnp")]
pub fn discover() -> Vec<String> {
    let mut ips = bridge::discover_upnp().unwrap();
    ips.dedup();
    ips
}

#[cfg(all(not(feature = "nupnp"), not(feature = "upnp")))]
pub fn discover() -> Vec<String> {
    panic!("Either UPnP or NUPnP is required for discovering!")
}
