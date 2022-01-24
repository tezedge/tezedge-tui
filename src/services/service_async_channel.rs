// Copyright (c) SimpleStaking, Viable Systems and Tezedge Contributors
// SPDX-License-Identifier: MIT

use std::sync::Arc;
use tokio::sync::mpsc;

pub type RequestTrySendError<T> = mpsc::error::TrySendError<T>;
pub type ResponseSendError<T> = mpsc::error::SendError<T>;

pub type ResponseTryRecvError = mpsc::error::TryRecvError;

/// Error while trying to receive next request from the requester.
///
/// Can only happen if requester is disconnected/shut down.
#[derive(Debug)]
pub struct RequestRecvError;

/// Requester half of the channel.
///
/// It is used to send requests to the worker.
#[derive(Debug)]
pub struct ServiceWorkerAsyncRequester<Req, Resp> {
    sender: mpsc::Sender<Req>,
    receiver: mpsc::Receiver<Resp>,
}

impl<Req, Resp> ServiceWorkerAsyncRequester<Req, Resp> {
    pub fn try_send(&self, req: Req) -> Result<(), RequestTrySendError<Req>> {
        Ok(self.sender.try_send(req)?)
    }

    pub fn try_recv(&mut self) -> Result<Resp, ResponseTryRecvError> {
        Ok(self.receiver.try_recv()?)
    }
}

#[inline(always)]
async fn responder_send<T>(
    sender: &mpsc::Sender<T>,
    msg: T,
) -> Result<(), ResponseSendError<T>> {
    sender.send(msg).await?;
    Ok(())
}

pub struct ServiceWorkerAsyncResponderSender<Resp> {
    sender: mpsc::Sender<Resp>,
}

impl<Resp> ServiceWorkerAsyncResponderSender<Resp> {
    pub async fn send(&self, resp: Resp) -> Result<(), ResponseSendError<Resp>> {
        responder_send(&self.sender, resp).await
    }
}

impl<Resp> Clone for ServiceWorkerAsyncResponderSender<Resp> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
        }
    }
}

/// Responder half of the channel.
///
/// It is used by worker to send responses to the requester.
pub struct ServiceWorkerAsyncResponder<Req, Resp> {
    sender: mpsc::Sender<Resp>,
    receiver: mpsc::Receiver<Req>,
}

impl<Req, Resp> ServiceWorkerAsyncResponder<Req, Resp> {
    pub fn sender(&self) -> ServiceWorkerAsyncResponderSender<Resp> {
        ServiceWorkerAsyncResponderSender {
            sender: self.sender.clone(),
        }
    }

    pub async fn send(&self, resp: Resp) -> Result<(), ResponseSendError<Resp>> {
        responder_send(&self.sender, resp).await
    }

    pub async fn recv(&mut self) -> Result<Req, RequestRecvError> {
        self.receiver.recv().await.ok_or(RequestRecvError {})
    }
}

pub fn worker_channel<Req, Resp>(
    bound: usize,
) -> (
    ServiceWorkerAsyncRequester<Req, Resp>,
    ServiceWorkerAsyncResponder<Req, Resp>,
) {
    let (requester_tx, responder_rx) = mpsc::channel(bound);
    let (responder_tx, requester_rx) = mpsc::channel(bound);

    (
        ServiceWorkerAsyncRequester {
            sender: requester_tx,
            receiver: requester_rx,
        },
        ServiceWorkerAsyncResponder {
            sender: responder_tx,
            receiver: responder_rx,
        },
    )
}
