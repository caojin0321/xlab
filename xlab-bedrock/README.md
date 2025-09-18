## xlab-Bedrock
This companion crate integrates AWS Bedrock as model provider with xlab.

## Usage

Add the companion crate to your `Cargo.toml`, along with the xlab-core crate:

```toml
[dependencies]
xlab-bedrock = "0.1.0"
xlab-core = "0.9.1"
```

You can also run `cargo add xlab-bedrock xlab-core` to add the most recent versions of the dependencies to your project.

See the [`/examples`](./examples) folder for usage examples.

Make sure to have AWS credentials env vars loaded before starting client such as:
```shell
export AWS_DEFAULT_REGION=us-east-1
export AWS_SECRET_ACCESS_KEY=.......
export AWS_ACCESS_KEY_ID=......
```
