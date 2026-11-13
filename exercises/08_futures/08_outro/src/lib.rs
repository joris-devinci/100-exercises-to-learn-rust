// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use patch::{
    data::{Ticket, TicketDraft, TicketPatch},
    store::{TicketId, TicketStore},
};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {
    pub async fn insert(&self, draft: TicketDraft) -> TicketId {
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        self.sender
            .send(Command::Insert {
                draft,
                response_channel: response_sender,
            })
            .await
            .unwrap();
        response_receiver.recv().await.unwrap()
    }

    pub async fn get(&self, id: TicketId) -> Option<Ticket> {
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        self.sender
            .send(Command::Get {
                id,
                response_channel: response_sender,
            })
            .await
            .unwrap();
        response_receiver.recv().await.unwrap()
    }

    pub async fn update(&self, patch: TicketPatch) {
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        self.sender
            .send(Command::Update {
                patch,
                response_channel: response_sender,
            })
            .await
            .unwrap();
        response_receiver.recv().await.unwrap();
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = mpsc::channel(capacity);
    tokio::spawn(server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
    Update {
        patch: TicketPatch,
        response_channel: Sender<()>,
    },
}

async fn server(mut receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    while let Some(command) = receiver.recv().await {
        match command {
            Command::Insert {
                draft,
                response_channel,
            } => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id).await;
            }
            Command::Get {
                id,
                response_channel,
            } => {
                let _ = response_channel.send(store.get(id).cloned()).await;
            }
            Command::Update {
                patch,
                response_channel,
            } => {
                let ticket = store.get_mut(patch.id).unwrap();
                ticket.description = patch
                    .description
                    .unwrap_or_else(|| ticket.description.clone());
                ticket.title = patch.title.unwrap_or_else(|| ticket.title.clone());
                ticket.status = patch.status.unwrap_or(ticket.status);
                let _ = response_channel.send(()).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use patch::data::Status;

    #[tokio::test]
    async fn inserts_gets_and_updates_a_ticket() {
        let client = launch(10);
        let id = client
            .insert(TicketDraft {
                title: "A title".try_into().unwrap(),
                description: "A description".try_into().unwrap(),
            })
            .await;

        assert_eq!(client.get(id).await.unwrap().status, Status::ToDo);
        client
            .update(TicketPatch {
                id,
                title: Some("Updated title".try_into().unwrap()),
                description: None,
                status: Some(Status::Done),
            })
            .await;

        let ticket = client.get(id).await.unwrap();
        assert_eq!(ticket.status, Status::Done);
        assert!(format!("{:?}", ticket.title).contains("Updated title"));
    }
}
