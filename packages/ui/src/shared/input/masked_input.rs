use crate::shared::input::InputType;
use dioxus::prelude::*;
#[cfg(feature = "web")]
use dioxus::web::WebEventExt;
#[cfg(feature = "web")]
use gloo::timers::future::TimeoutFuture;
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[cfg(feature = "web")]
use web_sys::{window, HtmlInputElement, InputEvent};

pub fn get_empty_mask(input_type: InputType) -> &'static str {
    if input_type == InputType::Phone {
        "(___) ___-____"
    } else if input_type == InputType::Zip {
        "_____"
    } else {
        ""
    }
}

pub fn get_max_len(input_type: InputType) -> usize {
    if input_type == InputType::Phone {
        10
    } else if input_type == InputType::Zip {
        5
    } else {
        0
    }
}

pub fn masked_use_effect_hook(
    id: &str,
    input_type: InputType,
    value: Signal<String>,
    required: Option<Memo<bool>>,
    mut masked_pattern: Signal<&'static str>,
    empty_mask: &'static str,
) {
    #[cfg(feature = "web")]
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(element) = document.get_element_by_id(id) {
                if let Ok(input) = element.dyn_into::<HtmlInputElement>() {
                    if value.read().is_empty() || value.read().as_str() == empty_mask {
                        if required.as_ref().map(|r| *r.read()).unwrap_or(false) {
                            let message = if input_type == InputType::Phone {
                                "Please enter a valid phone number"
                            } else {
                                "Please enter a valid zip code"
                            };
                            input.set_custom_validity(message);
                            masked_pattern.set(if input_type == InputType::Phone {
                                "\\(\\d{3}\\) \\d{3}-\\d{4}"
                            } else {
                                "\\d{5}"
                            });
                        } else {
                            input.set_custom_validity("");
                            masked_pattern.set(if input_type == InputType::Phone {
                                "\\(\\d{3}\\) \\d{3}-\\d{4}|\\(___\\) ___-____"
                            } else {
                                "\\d{5}|_____"
                            });
                        }
                    } else if value.read().contains('_') {
                        let message = if input_type == InputType::Phone {
                            "Please enter a valid phone number"
                        } else {
                            "Please enter a valid zip code"
                        };
                        input.set_custom_validity(message);
                    } else {
                        input.set_custom_validity("");
                    }
                }
            }
        }
    }
}

pub fn masked_onfocus_handler(
    e: Event<FocusData>,
    input_type: InputType,
    mut value: Signal<String>,
    empty_mask: &'static str,
) {
    #[cfg(feature = "web")]
    {
        if value.read().is_empty() || value.read().as_str() == empty_mask {
            value.set(empty_mask.to_string());
            if let Some(target) = e.as_web_event().target() {
                if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
                    let cursor_pos = if input_type == InputType::Phone { 1 } else { 0 };
                    spawn(async move {
                        TimeoutFuture::new(0).await;
                        let _ = input.set_selection_range(cursor_pos, cursor_pos);
                    });
                }
            }
        }
    }
}

pub fn masked_onblur_handler(mut value: Signal<String>, empty_mask: &'static str) {
    #[cfg(feature = "web")]
    {
        if value.read().contains('_') || value.read().as_str() == empty_mask {
            value.set("".to_string());
        }
    }
}

pub fn masked_oninput_handler(
    e: Event<FormData>,
    input_type: InputType,
    mut value: Signal<String>,
    max_len: usize,
) {
    #[cfg(feature = "web")]
    if let Some(target) = e.as_web_event().target() {
        if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
            let digits_before_cursor = e
                .value()
                .chars()
                .take(input.selection_start().ok().flatten().unwrap_or(0) as usize)
                .filter(|c| c.is_numeric())
                .count();

            let mut digits: String = e.value().chars().filter(|c| c.is_numeric()).collect();
            let old_val = value.peek().to_string();
            let old_digits_len = old_val.chars().filter(|c| c.is_numeric()).count();

            let is_delete = e
                .as_web_event()
                .dyn_ref::<InputEvent>()
                .map(|ie| ie.input_type().contains("delete"))
                .unwrap_or(false);

            if is_delete && digits.len() == old_digits_len && !digits.is_empty() {
                if digits_before_cursor > 0 && digits_before_cursor <= digits.len() {
                    digits.remove(digits_before_cursor - 1);
                }
            }

            let truncated: String = digits.chars().take(max_len).collect();
            let mut chars = truncated.chars();

            let formatted = if input_type == InputType::Phone {
                let mut result = String::with_capacity(14);
                result.push('(');
                for _ in 0..3 {
                    result.push(chars.next().unwrap_or('_'));
                }
                result.push_str(") ");
                for _ in 0..3 {
                    result.push(chars.next().unwrap_or('_'));
                }
                result.push('-');
                for _ in 0..4 {
                    result.push(chars.next().unwrap_or('_'));
                }
                result
            } else {
                let mut result = String::with_capacity(5);
                for _ in 0..5 {
                    result.push(chars.next().unwrap_or('_'));
                }
                result
            };

            value.set(formatted.clone());
            input.set_value(&formatted);

            let mut new_cursor = 0;
            let mut seen_digits = 0;
            for c in formatted.chars() {
                if seen_digits >= digits_before_cursor {
                    break;
                }
                if c.is_numeric() {
                    seen_digits += 1;
                }
                new_cursor += 1;
            }

            if input_type == InputType::Phone && new_cursor < 1 {
                new_cursor = 1;
            }

            let _ = input.set_selection_range(new_cursor as u32, new_cursor as u32);
        }
    }
}
