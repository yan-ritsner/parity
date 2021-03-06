// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

pub mod servers_set_change_session;
pub mod share_add_session;
pub mod share_change_session;
pub mod share_move_session;
pub mod share_remove_session;

mod sessions_queue;

use key_server_cluster::{SessionId, NodeId, SessionMeta};

/// Share change session metadata.
#[derive(Debug, Clone)]
pub struct ShareChangeSessionMeta {
	/// Key id.
	pub id: SessionId,
	/// Id of node, which has started this session.
	pub master_node_id: NodeId,
	/// Id of node, on which this session is running.
	pub self_node_id: NodeId,
}

impl ShareChangeSessionMeta {
	/// Convert to consensus session meta. `all_nodes_set` is the union of `old_nodes_set` && `new_nodes_set`.
	pub fn into_consensus_meta(self, all_nodes_set_len: usize) -> SessionMeta {
		SessionMeta {
			id: self.id,
			master_node_id: self.master_node_id,
			self_node_id: self.self_node_id,
			threshold: all_nodes_set_len - 1,
		}
	}
}
