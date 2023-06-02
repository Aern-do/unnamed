use std::{
    ffi::CString,
    fmt::{self, Display},
};

use llvm_sys::error::{
    LLVMConsumeError, LLVMDisposeErrorMessage, LLVMErrorRef, LLVMGetErrorMessage,
};

#[derive(Debug)]
pub struct ErrorMessage(*mut i8);
impl Drop for ErrorMessage {
    fn drop(&mut self) {
        unsafe { LLVMDisposeErrorMessage(self.0) }
    }
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = unsafe { CString::from_raw(self.0) };
        write!(f, "{}", message.to_string_lossy())
    }
}

#[derive(Debug)]
pub struct Error(pub(crate) LLVMErrorRef);

impl Error {
    pub fn get_error_message(self) -> ErrorMessage {
        unsafe {
            let error_message = LLVMGetErrorMessage(self.0);
            ErrorMessage(error_message)
        }
    }

    pub fn is_success(&self) -> bool {
        self.0.is_null()
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe { LLVMConsumeError(self.0) }
    }
}
