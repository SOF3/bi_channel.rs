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

use std::sync::mpsc::{channel, Receiver, RecvError, RecvTimeoutError, Sender, SendError, TryRecvError};
use std::time::Duration;

use crate::BiMessage;

/// Opposite endpoints of two pairs of channels to send and receive.
pub struct BiChannel<Sent, Received> {
    sender: Sender<Sent>,
    receiver: Receiver<Received>,
}

impl<Sent, Received> BiChannel<Sent, Received> {
    pub fn send(&self, send: Sent) -> Result<(), SendError<Sent>> { self.sender.send(send) }

    pub fn recv(&self) -> Result<Received, RecvError> { self.receiver.recv() }
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Received, RecvTimeoutError> { self.receiver.recv_timeout(timeout) }
    pub fn try_recv(&self) -> Result<Received, TryRecvError> { self.receiver.try_recv() }
}

pub type InternalBiChannel<M> = BiChannel<<M as BiMessage>::Out, <M as BiMessage>::In>;
pub type ExternalBiChannel<M> = BiChannel<<M as BiMessage>::In, <M as BiMessage>::Out>;

pub struct BiChannelPair<M: BiMessage> {
    pub internal: BiChannel<M::Out, M::In>,
    pub external: BiChannel<M::In, M::Out>,
}

/// Creates two pair of MPSC channels.
/// The communication involves an "internal" party and an "external" party.
/// The "internal" party holds the internal BiChannel to send Out messages and receive In messages.
/// Vice versa for the "external" party.
pub fn bi_channel<M: BiMessage>() -> BiChannelPair<M> {
    let (in_send, in_recv) = channel::<M::In>();
    let (out_send, out_recv) = channel::<M::Out>();
    BiChannelPair {
        internal: BiChannel { sender: out_send, receiver: in_recv },
        external: BiChannel { sender: in_send, receiver: out_recv },
    }
}
