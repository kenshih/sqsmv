# sqsmv

**STATUS: alpha**
![CI](https://github.com/kenshih/sqsmv/workflows/CI/badge.svg)
# Table of Contents

- [sqsmv](#sqsmv)
- [Table of Contents](#table-of-contents)
- [Description](#description)
- [Installation Options](#installation-options)
  - [Download the binary](#download-the-binary)
- [Usage](#usage)
- [Errors](#errors)
- [Development](#development)

# Description

Moves messages from one sqs queue into another queue e.g. replay dlq to non-dlq

It does so by:
1. reading the "from" sqs queue in a batch of 1-10 messages
2. writes to the target "to" queue in same batches ^
3. deletes from the origin "from" queue messages of the batches ^
4. repeats 1-3 until the origin queue is empty, an error occurs, or the max iterations (1m) is exceeded
# Installation Options
## Download the binary
1. Download the [v0.1.0-alpha binary](https://github.com/kenshih/sqsmv/releases/tag/v0.1.0-alpha) for MacOx or Linux
2. unzip it
3. you may need to `chmod +x sqsmv` to give execute prives
4. run it!
5. (Optionally) add it to your PATH somewhere
# Usage

Assuming you have [AWS Named Profiles](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html) set up and both queues are under the same subaccount, this is how to run `sqsmv` from the command line ( also see `sqsmv --help`).

```
AWS_PROFILE=my-profile \
  sqsmv \
  --from-q <FROM_SQS_QUEUE> \
  --to-q <TO_SQS_QUEUE>

# e.g. it should look something like this:
AWS_PROFILE=profile11111111 \
  sqsmv \
  -f'https://sqs.us-east-1.amazonaws.com/11111111/my-queue-dlq' \
  -t'https://sqs.us-east-1.amazonaws.com/11111111/my-queue'
```

# Errors

This program is very conservative with errors right now.

1. If any READs on the SQS fail, the program will exit, reporting the READ failure
2. If any WRITEs fail, the program will exit, reporting WRITE failures
3. If any DELETEs fail, the program will exit, reporting DELETE failures that user can then use to do any cleanup necessary

# Development

```
AWS_PROFILE=my-profile \
  cargo run -- \
  -f'https://sqs.us-east-1.amazonaws.com/<my-acct#>/my-queue-dlq' \
  -t'https://sqs.us-east-1.amazonaws.com/<my-acct#>/my-queue'
```