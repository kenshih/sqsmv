# sqsmv

**STATUS: in development**

Moves messages from one sqs queue into another queue e.g. replay dlq to non-dlq

It does so by:
1. reads "from" sqs queue in a loop, iterating in batches until sqs queue is empty
2. writes to the "to" queue in same batch ^
3. deletes "from" sqs queue messages of the batch ^

# Usage

```
AWS_PROFILE=my-profile \
  sqsmv \
  --from-q <FROM_SQS_QUEUE> \
  --to-q <TO_SQS_QUEUE>
```

# Errors

This program is very conservative with errors right now.

1. If any READs on the SQS fail, the program will exit, reporting the READ failure
2. If any WRITEs fail, the program will exit, reporting WRITE failures
3. If any DELETEs fail, the program will exit, reporting DELETE failures that user can then use to do any cleanup necessary

## in development

```
AWS_PROFILE=my-profile \
  cargo run -- \
  -f'https://sqs.us-east-1.amazonaws.com/<my-acct#>/my-queue-dlq' \
  -t'https://sqs.us-east-1.amazonaws.com/<my-acct#>/my-queue'
```