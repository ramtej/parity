use client::BlockChainClient;
use util::network::{HandlerIo, PeerId, PacketId,};
use util::error::UtilError;

/// IO interface for the syning handler.
/// Provides peer connection management and an interface to the blockchain client.
// TODO: ratings
pub trait SyncIo {
	/// Disable a peer
	fn disable_peer(&mut self, peer_id: &PeerId);
	/// Respond to current request with a packet. Can be called from an IO handler for incoming packet.
	fn respond(&mut self, packet_id: PacketId, data: Vec<u8>) -> Result<(), UtilError>;
	/// Send a packet to a peer.
	fn send(&mut self, peer_id: PeerId, packet_id: PacketId, data: Vec<u8>) -> Result<(), UtilError>;
	/// Get the blockchain
	fn chain<'s>(&'s mut self) -> &'s mut BlockChainClient;
}

/// Wraps `HandlerIo` and the blockchain client
pub struct NetSyncIo<'s, 'h> where 'h:'s {
	network: &'s mut HandlerIo<'h>,
	chain: &'s mut BlockChainClient
}

impl<'s, 'h> NetSyncIo<'s, 'h> {
	/// Creates a new instance from the `HandlerIo` and the blockchain client reference.
	pub fn new(network: &'s mut HandlerIo<'h>, chain: &'s mut BlockChainClient) -> NetSyncIo<'s,'h> {
		NetSyncIo {
			network: network,
			chain: chain,
		}
	}
}

impl<'s, 'h> SyncIo for NetSyncIo<'s, 'h> {
	fn disable_peer(&mut self, peer_id: &PeerId) {
		self.network.disable_peer(*peer_id);
	}

	fn respond(&mut self, packet_id: PacketId, data: Vec<u8>) -> Result<(), UtilError>{
		self.network.respond(packet_id, data)
	}

	fn send(&mut self, peer_id: PeerId, packet_id: PacketId, data: Vec<u8>) -> Result<(), UtilError>{
		self.network.send(peer_id, packet_id, data)
	}

	fn chain<'a>(&'a mut self) -> &'a mut BlockChainClient {
		self.chain
	}
}

