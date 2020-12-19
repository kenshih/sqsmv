#[macro_use]
extern crate clap;
use clap::App;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    // .unwrap() ok because FROM_SQS_QUEUE and TO_SQS_QUEUE are required (see cli.yml)
    let from_sqs = matches.value_of("FROM_SQS_QUEUE").unwrap();
    let to_sqs = matches.value_of("TO_SQS_QUEUE").unwrap();
    let message_mover = sqsmv::QueueMessageMover::new(from_sqs.to_string(),
        to_sqs.to_string());
    println!("Using\nFROM_SQS_QUEUE: {}", from_sqs);
    println!(" TO_SQS_QUEUE: {}", to_sqs);

    if matches.is_present("verbose") {
        println!("[verbosity enabled]");
    } 

    if matches.is_present("delete-source") {
        println!("[delete-source enabled]: will delete message on successful mv");
    }

    message_mover.run().await;
}
