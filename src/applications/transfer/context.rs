use crate::{context::Context, *};

use ibc::{
	applications::transfer::{
		context::{BankKeeper, Ics20Context, Ics20Keeper, Ics20Reader},
		error::Error as ICS20Error,
		PrefixedCoin, PrefixedDenom,
		PORT_ID_STR
	},
	core::ics24_host::identifier::{ChannelId as IbcChannelId, PortId},
	signer::Signer,
};

pub struct MocKAccountId;

impl From<Signer> for MocKAccountId {
	fn from(_sig: Signer) -> Self {
		MocKAccountId
	}
}

impl<T: Config> Ics20Context for Context<T> {
	type AccountId = MocKAccountId; // MOCK
}

impl<T: Config> Ics20Keeper for Context<T> {
	type AccountId = MocKAccountId;
}

impl<T: Config> Ics20Reader for Context<T> {
	type AccountId = MocKAccountId;

	fn get_port(&self) -> Result<PortId, ICS20Error> {
		PortId::from_str(PORT_ID_STR)
			.map_err(|e| ICS20Error::invalid_port_id(PORT_ID_STR.to_string(), e))
	}

	fn get_channel_escrow_address(
		&self,
		_port_id: &PortId,
		_channel_id: IbcChannelId,
	) -> Result<<Self as Ics20Reader>::AccountId, ICS20Error> {
		todo!()
	}

	fn is_send_enabled(&self) -> bool {
		todo!()
	}

	fn is_receive_enabled(&self) -> bool {
		todo!()
	}

	fn denom_hash_string(&self, _denom: &PrefixedDenom) -> Option<String> {
		todo!()
	}
}

impl<T: Config> BankKeeper for Context<T> {
	type AccountId = MocKAccountId;

	fn send_coins(
		&mut self,
		_from: &Self::AccountId,
		_to: &Self::AccountId,
		_amt: &PrefixedCoin,
	) -> Result<(), ICS20Error> {
		todo!()
	}

	fn mint_coins(
		&mut self,
		_account: &Self::AccountId,
		_amt: &PrefixedCoin,
	) -> Result<(), ICS20Error> {
		todo!()
	}
	
	fn burn_coins(
		&mut self,
		_account: &Self::AccountId,
		_amt: &PrefixedCoin,
	) -> Result<(), ICS20Error> {
		todo!()
	}
}
