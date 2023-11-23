mod client;

use ::drink::{
    errors::MessageResult, runtime::HashFor, session::error::SessionError, DispatchError, Weight,
};
pub use client::*;

use crate::{ContractEvent, ExecCall, InstantiateCall, ReadCall, UploadCall};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Drink error: {0}")]
    DrinkError(SessionError),
    #[error("Decoding error: {0}")]
    DecodingError(String),
    #[error("Code hash mismatch")]
    CodeHashMismatch,
    #[error("Deployment reverted")]
    DeploymentReverted,
    #[error("Deployment failed: {0:?}")]
    DeploymentFailed(DispatchError),
    #[error("Contract call reverted")]
    CallReverted,
    #[error("Contract call failed: {0:?}")]
    CallFailed(DispatchError),
}

impl From<SessionError> for Error {
    fn from(e: SessionError) -> Self {
        Self::DrinkError(e)
    }
}

pub trait Connection<R: frame_system::Config> {
    fn upload_code(&mut self, call: UploadCall) -> Result<HashFor<R>, Error>;

    fn instantiate<T: Send>(
        &mut self,
        call: InstantiateCall<T>,
    ) -> Result<ContractInstantiateResult<R::AccountId>, Error>;

    fn exec<T: scale::Decode + Send>(
        &mut self,
        call: ExecCall<T>,
    ) -> Result<ContractExecResult<MessageResult<T>>, Error>;

    // like `exec`, but does not commit changes
    fn read<T: scale::Decode + Send>(
        &mut self,
        call: ReadCall<T>,
    ) -> Result<ContractReadResult<MessageResult<T>>, Error>;
}

#[derive(Debug)]
pub struct ContractResult<R> {
    pub gas_consumed: Weight,
    pub gas_required: Weight,
    pub result: R,
    pub events: Vec<ContractEvent>,
}

impl<R: Clone> Clone for ContractResult<R> {
    fn clone(&self) -> Self {
        Self {
            gas_consumed: self.gas_consumed,
            gas_required: self.gas_required,
            result: self.result.clone(),
            events: self.events.clone(),
        }
    }
}

pub type ContractInstantiateResult<AccountId> = ContractResult<AccountId>;

pub type ContractExecResult<R> = ContractResult<R>;

pub type ContractReadResult<R> = ContractResult<R>;
