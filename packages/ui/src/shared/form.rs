pub mod sms_opt_in;

#[derive(Clone, PartialEq)]
pub enum FormStatus {
    Idle,
    Processing,
    Success,
    Error,
}
