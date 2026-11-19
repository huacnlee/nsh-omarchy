//! [`Field`]: a control's label, description and error.

use nsh_core::prelude::*;

use crate::tokens;

/// The props of [`Field`].
#[derive(Debug, Clone, PartialEq, Props)]
pub struct FieldProps {
    /// The label above the control.
    pub label: Prop<SharedString>,
    /// A line below the control on what it takes.
    pub description: Prop<Option<SharedString>>,
    /// What is wrong with the value: shown in the danger color in place of
    /// the description.
    pub error: Prop<Option<SharedString>>,
    /// Marks the label with a danger asterisk: the form needs a value.
    #[prop(default)]
    pub required: Prop<bool>,
    /// The control.
    pub children: Prop<Children>,
}

/// A control's label above it and a description or an error below:
/// `<Field label="Hostname" description="Return submits."><Input /></Field>`.
#[component]
pub fn Field(props: FieldProps) -> impl IntoElement {
    // The control inside is named by the label.
    provide_context(super::FieldLabel(props.label));
    let note = move || props.error.get().or_else(|| props.description.get());
    view()
        .gap(px(tokens::SPACE_1))
        .child(
            view()
                .flex_row()
                .gap(px(tokens::SPACE_1))
                .child(
                    text(move || props.label.get())
                        .text_size(px(tokens::TEXT_SM))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(move || tokens::secondary()),
                )
                .child(dynamic(move || {
                    props.required.get().then(|| {
                        text("*")
                            .text_size(px(tokens::TEXT_SM))
                            .text_color(move || tokens::danger())
                            .accessibility_elements_hidden(true)
                            .into_element()
                    })
                })),
        )
        .child(props.children)
        .child(dynamic(move || {
            note().map(|words| {
                text(words)
                    .text_size(px(tokens::TEXT_SM))
                    .text_color(move || {
                        if props.error.with(Option::is_some) {
                            tokens::danger()
                        } else {
                            tokens::secondary()
                        }
                    })
                    .into_element()
            })
        }))
}
