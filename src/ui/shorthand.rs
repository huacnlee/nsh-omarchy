//! The shorthand methods of enum props, as inherent methods (no trait to
//! import): `button("Save").primary().small()`. Each is exactly the prop's
//! setter with one value.

/// `.small()`, `.medium()`, `.large()` on elements with a `size:
/// ControlSize` prop.
#[macro_export]
#[doc(hidden)]
macro_rules! sizable {
    ($($element:ty),+ $(,)?) => {$(
        impl $element {
            /// 24 points tall: `.size(ControlSize::Small)`.
            pub fn small(self) -> Self {
                self.size($crate::ControlSize::Small)
            }
            /// 28 points tall, the default: `.size(ControlSize::Medium)`.
            pub fn medium(self) -> Self {
                self.size($crate::ControlSize::Medium)
            }
            /// 32 points tall: `.size(ControlSize::Large)`.
            pub fn large(self) -> Self {
                self.size($crate::ControlSize::Large)
            }
        }
    )+};
}

/// `.primary()`, `.ghost()`, `.danger()` on elements with a `variant:
/// ButtonVariant` prop.
#[macro_export]
#[doc(hidden)]
macro_rules! button_variants {
    ($($element:ty),+ $(,)?) => {$(
        impl $element {
            /// The accent outline with accent text, the action a view leads
            /// with: `.variant(ButtonVariant::Primary)`.
            pub fn primary(self) -> Self {
                self.variant($crate::ButtonVariant::Primary)
            }
            /// No outline until focused, for toolbars and dense rows:
            /// `.variant(ButtonVariant::Ghost)`.
            pub fn ghost(self) -> Self {
                self.variant($crate::ButtonVariant::Ghost)
            }
            /// The danger outline with danger text, for destructive actions:
            /// `.variant(ButtonVariant::Danger)`.
            pub fn danger(self) -> Self {
                self.variant($crate::ButtonVariant::Danger)
            }
        }
    )+};
}

/// `.horizontal()`, `.vertical()` on elements with a `layout: Axis` prop.
#[macro_export]
#[doc(hidden)]
macro_rules! axis_shorthands {
    ($($element:ty),+ $(,)?) => {$(
        impl $element {
            /// Side by side: `.layout(Axis::Horizontal)`.
            pub fn horizontal(self) -> Self {
                self.layout($crate::Axis::Horizontal)
            }
            /// One under another: `.layout(Axis::Vertical)`.
            pub fn vertical(self) -> Self {
                self.layout($crate::Axis::Vertical)
            }
        }
    )+};
}
