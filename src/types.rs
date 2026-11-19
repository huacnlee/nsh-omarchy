//! The types several components share.

use std::rc::Rc;

use nsh_core::element::{IntoProp, JsHost, JsIn, JsKind, JsOwned, JsValue, PropError, TsTypes};
use nsh_core::prelude::*;

pub use nsh_core::geometry::Axis;
pub use nsh_core::overlay::{Align, Placement};

/// A semantic color for a fill or an action: a progress bar, an alert
/// dialog's action.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PropEnum)]
#[non_exhaustive]
pub enum Tone {
    /// The theme's accent: the default.
    #[default]
    Accent,
    /// Done, healthy.
    Success,
    /// Needs attention.
    Warning,
    /// Destructive or failed.
    Danger,
}

/// A controlled value prop: set by the app (it owns the value and writes back
/// what `on_change` reports), or left out (the component keeps its own).
///
/// It is never written: the setters take the value itself
/// (`.value("dwindle")`, `.value(Some(4.0))`), a signal of it or a closure
/// returning it. In TSX it is the value, `undefined` when left out.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Controlled<T> {
    /// Left out: the component keeps its own value.
    Uncontrolled,
    /// The app's value.
    Set(T),
}

impl<T> Default for Controlled<T> {
    fn default() -> Self {
        Self::Uncontrolled
    }
}

impl<T: Clone> Controlled<T> {
    /// The app's value, or `None` while it is left out.
    pub fn get(&self) -> Option<T> {
        match self {
            Self::Uncontrolled => None,
            Self::Set(value) => Some(value.clone()),
        }
    }

    /// The app's value, else `own()`.
    pub(crate) fn or_else(&self, own: impl FnOnce() -> T) -> T {
        match self {
            Self::Uncontrolled => own(),
            Self::Set(value) => value.clone(),
        }
    }

    /// Left out.
    pub(crate) fn is_uncontrolled(&self) -> bool {
        matches!(self, Self::Uncontrolled)
    }
}

impl<T> From<T> for Controlled<T> {
    fn from(value: T) -> Self {
        Self::Set(value)
    }
}

impl From<&str> for Controlled<SharedString> {
    fn from(value: &str) -> Self {
        Self::Set(value.to_owned().into())
    }
}

impl From<String> for Controlled<SharedString> {
    fn from(value: String) -> Self {
        Self::Set(value.into())
    }
}

impl From<&str> for Controlled<Option<SharedString>> {
    fn from(value: &str) -> Self {
        Self::Set(Some(value.to_owned().into()))
    }
}

impl From<String> for Controlled<Option<SharedString>> {
    fn from(value: String) -> Self {
        Self::Set(Some(value.into()))
    }
}

impl From<SharedString> for Controlled<Option<SharedString>> {
    fn from(value: SharedString) -> Self {
        Self::Set(Some(value))
    }
}

impl From<Option<&str>> for Controlled<Option<SharedString>> {
    fn from(value: Option<&str>) -> Self {
        Self::Set(value.map(|v| v.to_owned().into()))
    }
}

impl From<Option<String>> for Controlled<Option<SharedString>> {
    fn from(value: Option<String>) -> Self {
        Self::Set(value.map(Into::into))
    }
}

impl From<Vec<&str>> for Controlled<Vec<SharedString>> {
    fn from(value: Vec<&str>) -> Self {
        Self::Set(value.into_iter().map(|v| v.to_owned().into()).collect())
    }
}

impl From<Vec<String>> for Controlled<Vec<SharedString>> {
    fn from(value: Vec<String>) -> Self {
        Self::Set(value.into_iter().map(Into::into).collect())
    }
}

impl From<f64> for Controlled<Option<f64>> {
    fn from(value: f64) -> Self {
        Self::Set(Some(value))
    }
}

/// `T`'s TypeScript type: `undefined` (left out) is the field's own value.
impl<T: JsValue> JsValue for Controlled<T> {
    fn ts_type(types: &mut TsTypes) -> String {
        T::ts_type(types)
    }

    fn from_js(value: JsIn<'_>) -> Result<Self, PropError> {
        match value.kind() {
            JsKind::Undefined => Ok(Self::Uncontrolled),
            _ => T::from_js(value).map(Self::Set),
        }
    }

    fn to_js<'a>(&self, host: &'a dyn JsHost) -> Result<JsOwned<'a>, PropError> {
        match self {
            Self::Uncontrolled => Ok(JsOwned::new(host, host.undefined())),
            Self::Set(value) => value.to_js(host),
        }
    }
}

/// How a signal of a field's value is its value prop ([`IntoProp`]).
#[doc(hidden)]
pub struct ControlledHandle(());

macro_rules! controlled_handles {
    ($($handle:ident),*) => {$(
        /// A signal of the value (or of what converts to it), as the app's
        /// value.
        impl<T, U> IntoProp<Controlled<T>, ControlledHandle> for $handle<U>
        where
            T: 'static,
            U: Clone + Into<T> + 'static,
        {
            fn into_prop(self) -> Reactive<Controlled<T>> {
                Reactive::Bound(Rc::new(move || Controlled::Set(self.get().into())))
            }
        }
    )*};
}

controlled_handles!(Signal, ReadSignal, Memo);
