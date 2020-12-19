use rusoto_core::{Region, RusotoError};
use rusoto_sqs::{
  DeleteMessageBatchError, DeleteMessageBatchRequest, DeleteMessageBatchRequestEntry,
  DeleteMessageBatchResult, Message, ReceiveMessageRequest, ReceiveMessageResult,
  SendMessageBatchError, SendMessageBatchRequest, SendMessageBatchRequestEntry,
  SendMessageBatchResult, Sqs, SqsClient,
};
use uuid::Uuid;

//#[derive(Debug)]
pub struct QueueMessageMover {
  from_queue_url: String,
  to_queue_url: String,
  client: SqsClient,
}

impl QueueMessageMover {
  pub fn new(from_queue_url: String, to_queue_url: String) -> Self {
    QueueMessageMover {
      from_queue_url,
      to_queue_url,
      client: SqsClient::new(Region::UsEast1),
    }
  }

  async fn clear_messages(
    &self,
    v: Vec<Message>,
  ) -> Result<DeleteMessageBatchResult, RusotoError<DeleteMessageBatchError>> {
    //v.iter().clone().map(|v| v.)
    let entries: Vec<DeleteMessageBatchRequestEntry> = v
      .clone()
      .iter()
      .map(|m| DeleteMessageBatchRequestEntry {
        id: Uuid::new_v4().to_string(),
        receipt_handle: m.receipt_handle.clone().unwrap(),
      })
      .collect();

    let req = DeleteMessageBatchRequest {
      queue_url: self.from_queue_url.clone(),
      entries,
    };
    self.client.delete_message_batch(req).await
  }

  async fn write_messages(
    &self,
    entries: Vec<SendMessageBatchRequestEntry>,
  ) -> Result<SendMessageBatchResult, RusotoError<SendMessageBatchError>> {
    let batch = SendMessageBatchRequest {
      queue_url: String::from(&self.to_queue_url),
      entries,
    };
    self.client.send_message_batch(batch).await
  }

  fn build_put_message_from_orig(&self, msg: &Message) -> SendMessageBatchRequestEntry {
    let uuid = Uuid::new_v4();
    //msg.attributes;
    SendMessageBatchRequestEntry {
      id: uuid.to_string(),
      delay_seconds: Some(0),
      message_attributes: None, //Some(msg.attributes),
      message_body: match msg.body.clone() {
        Some(b) => b,
        None => "".to_string(),
      },
      message_deduplication_id: None,
      message_group_id: None,
      message_system_attributes: None,
    }
  }

  //multi
  async fn handle_messages(&self, receive_message_result: ReceiveMessageResult) {
    match receive_message_result.messages {
      Some(vec) => {
        let send_messages = vec
          .iter()
          .clone()
          .map(|x| self.build_put_message_from_orig(x))
          .collect::<Vec<SendMessageBatchRequestEntry>>();

        match self.write_messages(send_messages).await {
          Ok(_) => { 
            match self.clear_messages(vec).await {
              Ok(x) => println!("cleanup succeeded {:?}", x),
              Err(e) => panic!("Could not delete message after post. Exiting. Messages from source not-deleted, yet posted to sink: {:?}", e),
            }
          ()},
          Err(e) => println!("swallowing error after write {}", e),
        };
      }
      None => println!("0 messages in result"),
    }
  }
  pub async fn run(&self) {
    //let client = SqsClient::new(Region::UsEast1);

    let receive_request = ReceiveMessageRequest {
      attribute_names: None,
      max_number_of_messages: Some(1),
      message_attribute_names: None,
      queue_url: String::from(&self.from_queue_url),
      receive_request_attempt_id: None,
      visibility_timeout: Some(10),
      wait_time_seconds: Some(1),
    };

    match self.client.receive_message(receive_request).await {
      Ok(result) => self.handle_messages(result).await,
      Err(err) => println!("{:?}", err),
    };
  }
}
