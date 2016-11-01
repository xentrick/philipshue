use hyper::Client;
use hyper::client::Body;

use serde_json::{to_string, from_reader, Map, Value};

use errors::HueError;
use ::hue::*;

/// Attempt to discover bridges using `https://www.meethue.com/api/nupnp`
pub fn discover() -> Result<Vec<Discovery>, HueError> {
    Client::new()
        .get("https://www.meethue.com/api/nupnp")
        .send()
        .map_err(HueError::from)
        .and_then(|ref mut r| from_reader(r).map_err(From::from))
}

/// Tries to register a user, returning the username if successful
///
/// This usually returns a `HueError::BridgeError` saying the link button needs to be pressed.
/// Therefore it recommended to call this function in a loop:
/// ## Example
/// ```no_run
/// use philipshue::errors::{HueError, BridgeError};
/// use philipshue::bridge::{self, Bridge};
///
/// let mut bridge = None;
/// // Discover a bridge
/// let bridge_ip = philipshue::bridge::discover().unwrap().pop().unwrap().into_ip();
/// let devicetype = "my_hue_app#homepc";
///
/// // Keep trying to register a user
/// loop{
///     match bridge::register_user(&bridge_ip, devicetype){
///         // A new user has succesfully been registered and the username is returned
///         Ok(username) => {
///             bridge = Some(Bridge::new(bridge_ip, username));
///             break;
///         },
///         // Prompt the user to press the link button
///         Err(HueError::BridgeError{error: BridgeError::LinkButtonNotPressed, ..}) => {
///             println!("Please, press the link on the bridge. Retrying in 5 seconds");
///             std::thread::sleep(std::time::Duration::from_secs(5));
///         },
///         // Some other error happened
///         Err(e) => {
///             println!("Unexpected error occured: {:?}", e);
///             break
///         }
///     }
/// }
/// ```
pub fn register_user(ip: &str, devicetype: &str) -> Result<String, HueError>{
    let client = Client::new();

    let body = format!("{{\"devicetype\": {:?}}}", devicetype);
    let body = body.as_bytes();
    let url = format!("http://{}/api", ip);
    let mut resp = try!(client.post(&url)
        .body(Body::BufBody(body, body.len()))
        .send());

    let rur = try!(from_reader::<_, Vec<HueResponse<User>>>(&mut resp)).pop().unwrap();

    if let Some(User{username}) = rur.success{
        Ok(username)
    }else if let Some(error) = rur.error{
        Err(error.into())
    }else{
        Err(HueError::MalformedResponse)
    }
}

#[derive(Debug, Clone)]
/// A light with an ID
pub struct IdentifiedLight {
    /// The ID number of this light
    pub id: usize,
    /// The light object
    pub light: Light,
}

#[derive(Debug)]
/// The bridge connection
pub struct Bridge {
    client: Client,
    /// The IP address of the bridge
    pub ip: String,
    /// The username for the user on the bridge
    pub username: String,
}

fn send<T: Deserialize>(rb: RequestBuilder) -> Result<T, HueError>{
    rb.send()
      .map_err(HueError::from)
      .and_then(|ref mut resp| from_reader::<_, T>(resp).map_err(From::from))
}

use serde::Deserialize;
use hyper::client::RequestBuilder;

impl Bridge {
    /// Creates a `Bridge` on the given IP with the given username
    pub fn new<S: Into<String>, U: Into<String>>(ip: S, username: U) -> Self{
        Bridge{
            client: Client::new(),
            ip: ip.into(),
            username: username.into()
        }
    }
    fn url(&self) -> String {
        format!("http://{}/api/{}/", self.ip, self.username)
    }
    fn put<T: Deserialize>(&self, request: &str, body: String) -> Result<T, HueError> {
        let url = self.url() + request;
        let body = ::clean::clean_json(body);
        let body = body.as_bytes();

        send(self.client
            .put(&url)
            .body(Body::BufBody(body, body.len())))
    }
    fn post<T: Deserialize>(&self, request: &str, body: String) -> Result<T, HueError> {
        let url = self.url() + request;
        let body = ::clean::clean_json(body);
        let body = body.as_bytes();

        send(self.client
            .post(&url)
            .body(Body::BufBody(body, body.len()))
        )
    }
    fn get<T: Deserialize>(&self, request: &str) -> Result<T, HueError> {
        send(self.client.get(&format!("{}{}", self.url(), request)))
    }
    fn delete<T: Deserialize>(&self, request: &str) -> Result<T, HueError> {
        send(self.client.delete(&format!("{}{}", self.url(), request)))
    }
    // TODO search for new lights
    // TODO get new lights
    // TODO rename light
    // TODO delete lights?
    /// Gets all lights that are connected to the bridge
    pub fn get_all_lights(&self) -> Result<Map<usize, Light>, HueError> {
        self.get("lights")
    }
    /// Gets the light with the specific id
    pub fn get_light(&self, id: usize) -> Result<Light, HueError> {
        self.get(&format!("lights/{}", id))
    }
    /// Sets the state of a light by sending a `LightCommand` to the bridge for this light
    pub fn set_light_state(&self, id: usize, command: LightCommand) -> Result<Vec<HueResponse<Value>>, HueError>{
        self.put(&format!("lights/{}/state", id), try!(to_string(&command)))
    }
}
