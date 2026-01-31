use crate::shared::input::InputType;
use dioxus::prelude::*;
#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
#[cfg(feature = "web")]
use gloo::timers::future::TimeoutFuture;
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::HtmlInputElement;
#[cfg(feature = "web")]
use web_sys::InputEvent;

pub fn get_empty_mask(r#type: InputType) -> String {
    match r#type {
        InputType::Phone => "(___) ___-____".to_string(),
        InputType::Zip => "_____".to_string(),
        _ => "".to_string(),
    }
}

pub fn get_max_len(r#type: InputType) -> usize {
    match r#type {
        InputType::Phone => 10,
        InputType::Zip => 5,
        _ => 0,
    }
}

#[cfg(feature = "web")]
pub fn masked_use_effect_hook(
    input: &HtmlInputElement,
    r#type: InputType,
    required: Memo<bool>,
    value: Signal<String>,
    mut masked_pattern: Signal<String>,
    empty_mask: String,
) {
    // set custom validity message if empty
    if *value.read() == "".to_string() || *value.read() == empty_mask {
        // if required, set pattern to enforce full input and set validity message
        if *required.read() {
            masked_pattern.set(match r#type {
                InputType::Phone => "\\(\\d{3}\\) \\d{3}-\\d{4}".to_string(),
                InputType::Zip => "\\d{5}".to_string(),
                _ => "".to_string(),
            });
            let message = match r#type {
                InputType::Phone => "Please enter a valid phone number",
                InputType::Zip => "Please enter a valid zip code",
                _ => "",
            };
            input.set_custom_validity(message);
        } else {
            // if not required, set pattern to allow empty input and clear validity message
            masked_pattern.set(match r#type {
                InputType::Phone => "\\(\\d{3}\\) \\d{3}-\\d{4}|\\(___\\) ___-____".to_string(),
                InputType::Zip => "\\d{5}|_____".to_string(),
                _ => "".to_string(),
            });
            input.set_custom_validity("");
        }
    } else if value.read().contains('_') {
        // set custom validity message if incomplete but not empty regardless of required
        let message = match r#type {
            InputType::Phone => "Please enter a valid phone number",
            InputType::Zip => "Please enter a valid zip code",
            _ => "",
        };
        input.set_custom_validity(message);
    } else {
        // clear custom validity message if complete
        input.set_custom_validity("");
    }
}

#[cfg(feature = "web")]
pub fn masked_onfocus_handler(
    input: &HtmlInputElement,
    r#type: InputType,
    mut value: Signal<String>,
    empty_mask: String,
) {
    // if the input is empty, set it to the empty mask and position the cursor appropriately
    if *value.read() == "".to_string() || *value.read() == empty_mask {
        value.set(empty_mask.to_string());
        let cursor_pos = match r#type {
            InputType::Phone => 1,
            InputType::Zip => 0,
            _ => 0,
        };

        // set cursor position asynchronously to ensure it occurs after focus event
        let input = input.clone();
        spawn(async move {
            TimeoutFuture::new(0).await;
            let _ = input.set_selection_range(cursor_pos, cursor_pos);
        });
    }
}

#[cfg(feature = "web")]
pub fn masked_onblur_handler(mut value: Signal<String>, empty_mask: String) {
    // if the input matches the empty mask or is incomplete, clear it when it loses focus
    if value.read().contains('_') || *value.read() == empty_mask {
        value.set("".to_string());
    }
}

#[cfg(feature = "web")]
pub fn masked_oninput_handler(
    event: Event<FormData>,
    input: &HtmlInputElement,
    r#type: InputType,
    mut value: Signal<String>,
    max_len: usize,
) {
    // get digits before cursor position
    let digits_before_cursor = input
        .value()
        .chars()
        .take(if let Some(pos) = input.selection_start().ok().flatten() {
            pos as usize
        } else {
            0
        })
        .filter(|char| char.is_numeric())
        .count();

    // get all digits from input value
    let mut digits: String = event
        .value()
        .chars()
        .filter(|char| char.is_numeric())
        .collect();

    let old_val = value.peek().to_string();
    let old_digits_len = old_val.chars().filter(|char| char.is_numeric()).count();
    let is_delete = if let Some(input_event) = event.as_web_event().dyn_ref::<InputEvent>() {
        input_event.input_type().contains("delete")
    } else {
        false
    };

    // if deletion and no change in digit count, remove digit before cursor
    if is_delete && digits.len() == old_digits_len && !digits.is_empty() {
        if digits_before_cursor > 0 && digits_before_cursor <= digits.len() {
            digits.remove(digits_before_cursor - 1);
        }
    }

    // truncate digits to max length
    let truncated: String = digits.chars().take(max_len).collect();
    let mut chars = truncated.chars();

    // build formatted string based on input type
    let formatted = match r#type {
        InputType::Phone => {
            // format as (XXX) XXX-XXXX
            let mut result = String::with_capacity(14);
            result.push('(');
            for _ in 0..3 {
                result.push(if let Some(character) = chars.next() {
                    character
                } else {
                    '_'
                });
            }
            result.push_str(") ");
            for _ in 0..3 {
                result.push(if let Some(character) = chars.next() {
                    character
                } else {
                    '_'
                });
            }
            result.push('-');
            for _ in 0..4 {
                result.push(if let Some(character) = chars.next() {
                    character
                } else {
                    '_'
                });
            }
            result
        }
        InputType::Zip => {
            // format as XXXXX
            let mut result = String::with_capacity(5);
            for _ in 0..5 {
                result.push(if let Some(character) = chars.next() {
                    character
                } else {
                    '_'
                });
            }
            result
        }
        _ => "".to_string(),
    };

    // update value and input element
    value.set(formatted.clone());
    input.set_value(&formatted);

    // calculate new cursor position after formatting
    let mut new_cursor = 0;
    let mut seen_digits = 0;
    for c in formatted.chars() {
        // stop when we've seen as many digits as were before the cursor
        if seen_digits >= digits_before_cursor {
            break;
        }

        // only count numeric characters towards cursor position
        if c.is_numeric() {
            seen_digits += 1;
        }

        // always move cursor forward
        new_cursor += 1;
    }

    match r#type {
        // skip past opening parenthesis for phone numbers
        InputType::Phone if new_cursor < 1 => {
            new_cursor = 1;
        }
        _ => {}
    }

    // set new cursor position
    let _ = input.set_selection_range(new_cursor as u32, new_cursor as u32);
}
