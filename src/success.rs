use bridge::Success;

use hue::JsonValue;

/// Light change success
#[derive(Debug)]
pub struct Light {
    /// ID of the light changed
    id: usize,
    /// State changes
    state: State,
}

impl From<Success> for Light {
    fn from(v: Success) -> Self {
        let (id, state);

        let mut s = v.0.split('/');
        assert_eq!(s.next(), Some(""));
        assert_eq!(s.next(), Some("lights"));
        id = s.next().unwrap().parse().unwrap();
        assert_eq!(s.next(), Some("state"));
        state = s.next().unwrap();

        let state = match (state, v.1) {
            ("on", JsonValue::Bool(b)) => State::On(b),
            ("bri", JsonValue::Number(n)) => State::Bri(n.as_u64().unwrap() as u8),
            ("hue", JsonValue::Number(n)) => State::Hue(n.as_u64().unwrap() as u16),
            ("sat", JsonValue::Number(n)) => State::Sat(n.as_u64().unwrap() as u8),
            ("xy", JsonValue::Array(v)) => State::Xy(v[0].as_f64().unwrap() as f32, v[1].as_f64().unwrap() as f32),
            ("ct", JsonValue::Number(n)) => State::Ct(n.as_u64().unwrap() as u16),
            ("effect", JsonValue::String(s)) => State::Effect(s.to_owned()),
            (state, val) => State::Other{state: state.to_owned(), val}
        };

        Light {
            id,
            state
        }
    }
}

/// Group state change success
#[derive(Debug)]
pub struct GroupState {
    /// ID of the group changed
    id: usize,
    /// State changes
    state: State,
}
/* TODO Copied from `Light`
impl From<Success> for GroupState {
    fn from(v: Success) -> Self {
        let (id, state);

        let mut s = v.0.split('/');
        assert_eq!(s.next(), Some(""));
        assert_eq!(s.next(), Some("lights"));
        id = s.next().unwrap().parse().unwrap();
        assert_eq!(s.next(), Some("state"));
        state = s.next().unwrap();

        let state = match (state, v.1) {
            ("on", JsonValue::Bool(b)) => State::On(b),
            ("bri", JsonValue::Number(n)) => State::Bri(n.as_u64().unwrap() as u8),
            ("hue", JsonValue::Number(n)) => State::Hue(n.as_u64().unwrap() as u16),
            ("sat", JsonValue::Number(n)) => State::Sat(n.as_u64().unwrap() as u8),
            ("xy", JsonValue::Array(v)) => State::Xy(v[0].as_f64().unwrap() as f32, v[1].as_f64().unwrap() as f32),
            ("ct", JsonValue::Number(n)) => State::Ct(n.as_u64().unwrap() as u16),
            ("effect", JsonValue::String(s)) => State::Effect(s.to_owned()),
            (state, val) => State::Other{state: state.to_owned(), val}
        };

        GroupState {
            id,
            state
        }
    }
}
*/

/// State changes
#[derive(Debug)]
pub enum State {
    /// Light on/off change
    On(bool),
    /// Brightness change
    Bri(u8),
    /// Hue change
    Hue(u16),
    /// Saturation change
    Sat(u8),
    /// XY colour change
    Xy(f32, f32),
    /// Colour temperature change
    Ct(u16),
    /// Effect change
    Effect(String),
    /// ?
    Other{
        state: String,
        val: JsonValue
    }
}

/// Light rename success
#[derive(Debug)]
pub struct Rename {
    /// ID of the light changed
    id: usize,
    /// New name
    name: String,
}

impl From<Success> for Rename {
    fn from(v: Success) -> Self {
        let (id, name);

        let mut s = v.0.split('/');
        assert_eq!(s.next(), Some(""));
        assert_eq!(s.next(), Some("lights"));
        id = s.next().unwrap().parse().unwrap();
        assert_eq!(s.next(), Some("name"));
        name = v.1.as_str().unwrap().to_owned();

        Rename {
            id,
            name
        }
    }
}

/// Light delete success
#[derive(Debug)]
pub struct Delete {
    /// ID of the light deleted
    id: usize,
}

impl From<String> for Delete {
    fn from(s: String) -> Self {
        let (lights, id_deleted) = s.split_at(8);
        assert!(lights == "/lights/" || lights == "/groups/");
        let space = id_deleted.find(' ').unwrap();
        let (id, deleted) = id_deleted.split_at(space);
        assert_eq!(deleted, "deleted");

        Delete {
            id: id.parse().unwrap()
        }
    }
}

/// Group change success
#[derive(Debug)]
pub struct Group {
    /// ID of the group changed
    id: usize,
    /// Atributes changed
    attrib: Attrib,
}

impl From<Success> for Group {
    fn from(v: Success) -> Self {
        let (id, attrib);

        let mut s = v.0.split('/');
        assert_eq!(s.next(), Some(""));
        assert_eq!(s.next(), Some("groups"));
        id = s.next().unwrap().parse().unwrap();
        attrib = s.next().unwrap();

        let attrib = match (attrib, v.1) {
            ("name", JsonValue::String(n)) => Attrib::Name(n),
            ("class", JsonValue::String(c)) => Attrib::Class(c),
            ("lights", JsonValue::Array(v)) => Attrib::Lights(v.iter().map(|u| u.as_u64().unwrap() as usize).collect()),
            (a, b) => panic!("{:?}: {:?} didn't make sense", a, &b)
        };

        Group {
            id,
            attrib
        }
    }
}

/// Group attribute changes
#[derive(Debug)]
pub enum Attrib {
    /// New name
    Name(String),
    /// New lights in group
    Lights(Vec<usize>),
    /// New class
    Class(String),
}
