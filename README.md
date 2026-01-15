# pubd
A gRPC server for experimenting.

## countah
A service that streams a counter that counts up from 0, incrementing the value once a second, while the client is connected.
```bash
podman run --rm -it --network host -v ./proto:/proto:ro docker.io/fullstorydev/grpcurl:latest-alpine -plaintext -import-path /proto -proto countah.proto -d '{}' localhost:60069 unit.countah.v0.Countah.Counter
```
