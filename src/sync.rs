/*
 * bi_channel
 *
 * Copyright (C) 2019 SOFe
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::sync::mpsc::{Receiver, RecvError, RecvTimeoutError, SendError, sync_channel, SyncSender, TryRecvError, TrySendError};
use std::time::Duration;

use crate::BiMessage;

pub struct SyncBiChannel<Sent, Received> {
    pub sender: SyncSender<Sent>,
    pub receiver: Receiver<Received>,
}

impl<Sent, Received> SyncBiChannel<Sent, Received> {
    pub fn send(&self, send: Sent) -> Result<(), SendError<Sent>> { self.sender.send(send) }
    pub fn try_send(&self, send: Sent) -> Result<(), TrySendError<Sent>> { self.sender.try_send(send) }

    pub fn recv(&self) -> Result<Received, RecvError> { self.receiver.recv() }
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Received, RecvTimeoutError> { self.receiver.recv_timeout(timeout) }
    pub fn try_recv(&self) -> Result<Received, TryRecvError> { self.receiver.try_recv() }
}

pub type InternalSyncBiChannel<M> = SyncBiChannel<<M as BiMessage>::In, <M as BiMessage>::Out>;
pub type ExternalSyncBiChannel<M> = SyncBiChannel<<M as BiMessage>::Out, <M as BiMessage>::In>;

pub struct SyncBiChannelPair<M: BiMessage> {
    pub internal: SyncBiChannel<M::Out, M::In>,
    pub external: SyncBiChannel<M::In, M::Out>,
}

pub fn sync_bi_channel<M: BiMessage>(in_bound: usize, out_bound: usize) -> SyncBiChannelPair<M> {
    let (in_send, in_recv) = sync_channel::<M::In>(in_bound);
    let (out_send, out_recv) = sync_channel::<M::Out>(out_bound);
    SyncBiChannelPair {
        internal: SyncBiChannel { sender: out_send, receiver: in_recv },
        external: SyncBiChannel { sender: in_send, receiver: out_recv },
    }
}
