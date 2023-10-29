# ERC4626 SPS


``` export STREAMINGFAST_KEY=server_2fa5293455548fbbf440f013f2a6b657 # Use your own API key```

```export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)```