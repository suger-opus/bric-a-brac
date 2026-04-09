use crate::{application::AppError, infrastructure::InfraError};
use bric_a_brac_dtos::DtosConversionError;
use tonic::{Code, Status};

#[derive(Debug, thiserror::Error)]
pub enum PresentationError {
    #[error(transparent)]
    AppError(#[from] AppError),

    #[error(transparent)]
    DtosConversionError(#[from] DtosConversionError),
}

impl From<PresentationError> for Status {
    fn from(err: PresentationError) -> Self {
        match err {
            PresentationError::AppError(app_err) => app_err.into(),
            PresentationError::DtosConversionError(dto_err) => {
                tracing::warn!(error = ?dto_err);
                Self::new(Code::InvalidArgument, format!("{dto_err}"))
            }
        }
    }
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::InfraError(infra_err) => infra_err.into(),
            AppError::AgentError(agent_error) => {
                tracing::error!(error = ?agent_error);
                Self::new(Code::Internal, format!("{agent_error}"))
            }
            AppError::FileParsing { .. } => {
                tracing::warn!(error = ?err);
                Self::new(Code::InvalidArgument, format!("{err}"))
            }
        }
    }
}

impl From<InfraError> for Status {
    fn from(err: InfraError) -> Self {
        match err {
            InfraError::GrpcRequestError(grpc_err) => grpc_err.into(),
            InfraError::OpenRouterClientError(open_err) => {
                tracing::warn!(error = ?open_err);
                Self::new(Code::Unavailable, format!("{open_err}"))
            }
            InfraError::DtosConversionError(dtos_err) => {
                tracing::error!(error = ?dtos_err);
                Self::new(Code::Internal, format!("{dtos_err}"))
            }
        }
    }
}
