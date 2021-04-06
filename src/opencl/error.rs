use opencl3::{device::DeviceInfo, error_codes::ClError, program::ProgramInfo};

#[derive(thiserror::Error, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum GPUError {
    #[error("Opencl3 Error: {0}")]
    Opencl3(ClError),
    #[error("Device not found!")]
    DeviceNotFound,
    #[error("Device info not available!")]
    DeviceInfoNotAvailable(DeviceInfo),
    #[error("Program info not available!")]
    ProgramInfoNotAvailable(ProgramInfo),
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Cannot get bus ID for device with vendor {0}")]
    DeviceBusId(String),
}

#[allow(clippy::upper_case_acronyms)]
#[allow(dead_code)]
pub type GPUResult<T> = std::result::Result<T, GPUError>;

impl From<ClError> for GPUError {
    fn from(error: ClError) -> Self {
        GPUError::Opencl3(error)
    }
}
