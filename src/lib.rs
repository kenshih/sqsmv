use rusoto_core::Region;
use rusoto_sqs::{SqsClient, Sqs, ListQueuesRequest};

#[derive(Debug)]
pub struct Qs {
  pub from_queue_url: String,
  pub to_queue_url: String
}
pub async fn run(_qs: Qs) {
  let client = SqsClient::new(Region::UsEast1);
  let list_queues_request: ListQueuesRequest = 
    ListQueuesRequest {
      max_results: Some(3),
      next_token: None,
      queue_name_prefix: None
    }
    ; //Default::default(); 
  match client.list_queues(list_queues_request).await {
    Ok(result) => println!("{:?}", result),
    Err(err) => println!("{:?}", err),
  }
}