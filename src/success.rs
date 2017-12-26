use bridge::Success;

use hue::JsonValue;

pub(crate) fn convert<T: From<Success>>(sv: Vec<Success>) -> Vec<T> {
    let mut v = Vec::with_capacity(sv.len());
    v.extend(sv.into_iter().map(|s| s.into()));
    v
}

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
            (state, val) => State::Other{state: state.to_owned(), val}
        };

        Light {
            id,
            state
        }
    }
}

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
    /// ?
    Other{
        state: String,
        val: JsonValue
    }
}
