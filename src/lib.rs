use rusoto_core::{Region, RusotoError};
use rusoto_sqs::{
  DeleteMessageBatchError, DeleteMessageBatchRequest, DeleteMessageBatchRequestEntry,
  DeleteMessageBatchResult, Message, ReceiveMessageRequest, ReceiveMessageResult,
  SendMessageBatchError, SendMessageBatchRequest, SendMessageBatchRequestEntry,
  SendMessageBatchResult, Sqs, SqsClient,
};
use uuid::Uuid;

fn build_put_message_from_orig(msg: &Message) -> SendMessageBatchRequestEntry {
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

  async fn handle_messages(
    &self,
    receive_message_result: ReceiveMessageResult,
  ) -> Result<u8, String> {
    match receive_message_result.messages {
      Some(vec) => {
        // prep messages
        let send_messages = vec
          .iter()
          .clone()
          .map(|x| build_put_message_from_orig(x))
          .collect::<Vec<SendMessageBatchRequestEntry>>();
        // write them
        match self.write_messages(send_messages).await {
          Ok(_) => {
            // delete them
            match self.clear_messages(vec).await {
              Ok(x) => Ok(x.successful.len() as u8),//println!("cleanup succeeded {:?}", x),
              Err(e) => Err(format!("Could not delete message after post. Exiting. Messages from source not-deleted, yet posted to sink: {:?}", e)),
            }
          }
          Err(e) => Err(format!("swallowing error after write {}", e)),
        }
      }
      None => Ok(0),
    }
  }

  pub async fn receive_batch(&self) -> u8 {
    let receive_request = ReceiveMessageRequest {
      attribute_names: None,
      max_number_of_messages: Some(10),
      message_attribute_names: None,
      queue_url: String::from(&self.from_queue_url),
      receive_request_attempt_id: None,
      visibility_timeout: Some(10),
      wait_time_seconds: Some(1),
    };

    let processed_count = match self.client.receive_message(receive_request).await {
      Ok(result) => match self.handle_messages(result).await {
        Ok(count) => count,
        _ => 0,
      },
      Err(err) => {
        println!("{:?}", err);
        0
      }
    };
    processed_count
  }

  pub async fn run(&self) -> u32 {
    let mut count = 100;
    let mut total_count: u32 = 0;
    const MAX_LOOP_COUNT: u32 = 1000000;
    let mut loop_count = 0;
    while count > 0 && loop_count < MAX_LOOP_COUNT {
      count = self.receive_batch().await;
      println!("processed: {}", count);
      loop_count += 1;
      total_count += count as u32;
    }
    println!("total processed: {}", total_count);
    total_count
  }
}
