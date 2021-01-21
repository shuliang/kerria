#!/bin/bash

# database is just starting up, may not accept connections right away
sleep 5
# Run database migration
sqlx migrate info
sqlx migrate run

# Run server with cargo watch and systemfd for autoreload
systemfd --no-pid -s http::0.0.0.0:3000 -- cargo watch -x run
