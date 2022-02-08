cargo build --release --target x86_64-unknown-linux-gnu --package stm_graphql && strip ./target/x86_64-unknown-linux-gnu/release/stm_graphql && cp ./target/x86_64-unknown-linux-gnu/release/stm_graphql ./bootstrap && zip stm_graphql.zip bootstrap && rm bootstrap && aws lambda update-function-code --region us-east-1 --function-name stm_graphql --zip-file fileb://stm_graphql.zip