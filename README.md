# sqsmv

**STATUS: in development**

Moves messages from one sqs queue into another queue e.g. replay dlq to non-dlq

# Usage

```
AWS_PROFILE=my-profile \
  sqsmv \
  --from-q <FROM_SQS_QUEUE> \
  --to-q <TO_SQS_QUEUE>
```

## in development

```
AWS_PROFILE=my-profile \
  cargo run -- \
  -f'https://sqs.us-east-1.amazonaws.com/<my-acct#>/my-queue-dlq' \
  -t'https://sqs.us-east-1.amazonaws.com/<my-acct#>/my-queue'
```