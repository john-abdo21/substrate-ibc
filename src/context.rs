use crate::Config;
use sp_std::marker::PhantomData;

use ibc::{applications::transfer::MODULE_ID_STR, core::ics26_routing::context::RouterBuilder};
use pallet_ics20_transfer::ics20_callback::IbcTransferModule;

/// A struct capturing all the functional dependencies (i.e., context)
/// which the ICS26 module requires to be able to dispatch and process IBC messages.
use crate::module::core::ics26_routing::{Router, SubstrateRouterBuilder};
#[cfg(test)]
use ibc::{
	core::{
		ics02_client::{client_type::ClientType, context::ClientKeeper},
		ics24_host::identifier::ClientId,
	},
	mock::client_state::{client_type as mock_client_type, MockClientState},
	Height,
};

#[derive(Clone, Debug)]
pub struct Context<T: Config> {
	pub _pd: PhantomData<T>,
	pub router: Router,
}

impl<T: Config> Context<T> {
	pub fn new() -> Self {
		let r = SubstrateRouterBuilder::default()
			.add_route(MODULE_ID_STR.parse().unwrap(), IbcTransferModule(PhantomData::<T>)) // register transfer Module
			.unwrap()
			.build();

		Self { _pd: PhantomData::default(), router: r }
	}

	/// Associates a client record to this context.
	/// Given a client id and a height, registers a new client in the context and also associates
	/// to this client a mock client state and a mock consensus state for height `height`. The type
	/// of this client is implicitly assumed to be Mock.
	#[cfg(test)]
	pub fn with_client(self, client_id: &ClientId, height: Height) -> Self {
		self.with_client_parametrized(client_id, height, Some(mock_client_type()), Some(height))
	}

	/// Similar to `with_client`, this function associates a client record to this context, but
	/// additionally permits to parametrize two details of the client. If `client_type` is None,
	/// then the client will have type Mock, otherwise the specified type. If
	/// `consensus_state_height` is None, then the client will be initialized with a consensus
	/// state matching the same height as the client state (`client_state_height`).
	#[cfg(test)]
	pub fn with_client_parametrized(
		mut self,
		client_id: &ClientId,
		client_state_height: Height,
		client_type: Option<ClientType>,
		consensus_state_height: Option<Height>,
	) -> Self {
		use ibc::{
			core::ics02_client::{client_state::ClientState, consensus_state::ConsensusState},
			mock::{
				client_state::MOCK_CLIENT_TYPE, consensus_state::MockConsensusState,
				header::MockHeader,
			},
		};

		let cs_height = consensus_state_height.unwrap_or(client_state_height);

		let client_type = client_type.unwrap_or_else(mock_client_type);
		let (client_state, consensus_state) = if client_type.as_str() == MOCK_CLIENT_TYPE {
			(
				Some(MockClientState::new(MockHeader::new(client_state_height)).into_box()),
				MockConsensusState::new(MockHeader::new(cs_height)).into_box(),
			)
		} else {
			panic!("unknown client type")
		};

		self.store_client_type(client_id.clone(), client_type).unwrap();
		self.store_client_state(client_id.clone(), client_state.unwrap()).unwrap();
		self.store_consensus_state(client_id.clone(), cs_height, consensus_state)
			.unwrap();

		self
	}
}

impl<T: Config> Default for Context<T> {
	fn default() -> Self {
		Self::new()
	}
}
