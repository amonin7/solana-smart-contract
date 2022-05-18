# Contract on Solana Network

## Deploy notes

### Test on localhost network

`anchor test`

### Deploy on devnet

1. `solana config set --url devnet`

2. **Make sure you're on devnet.**\
`solana config get`

3. `anchor build`

4. **Get the new program id.**\
`solana address -k target/deploy/contract-keypair.json`

5. **Update Anchor.toml and lib.rs w/ new program id.**
6. **Make sure Anchor.toml is on devnet.**

7. **Build again.**\
`anchor build`

8. **And finally, you're good to deploy :)! Go ahead and run:**\
`anchor deploy`