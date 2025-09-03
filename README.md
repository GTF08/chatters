# Chatters
A simple microservice-based Rust chatting application. It uses axum server on backend, and react-like yew framework on frontend.

## Usage
### Backend
Backend requires setting up:
- Postgresql machine (ORMs were not used, sql queries are written to be POSTGRE compatible);
- Redis machine;
- Set up environment variables for database and redis connections and private/public keys.
Every microservice can be started by running either running executable or:
> cargo run --bin [container_name]

### Frontend
Frontend requires to have trunk server installed. It can be launched with following command:
>trunk serve

## NOTES
Current configuration is clunky to use. For future implementation it is required to:
- Move listen addreses/ports to environment variables;
- Add proper error handling and recovery;
- Frontend is not ready to be used, for now is written only to test server/client interactions;
- Write proper tests.
