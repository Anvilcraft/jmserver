# JMServer testing environment
This directory contains a setup that can be used to run the JMServer for debugging.

It contains a `docker-compose.yml` file which can run everything needed for JMServer in docker containers,
and also a script to run JMServer configured to use these containers.

## How to use
1. Run `docker-compose up` in the testenv directory, and wait for all the containers to start.
2. Run the `debug_run.sh` shellscript which will start JMServer.

## Infos
The docker compose file contains these containers:
1. A mariadb databse with a `jensmemes` user, who has the password `snens`. This database is set-up with the scheme and some example data.
2. A caddy HTTP server as a CDN. It serves just `/0/uff.png` as an example meme.
3. An adminer admin interface for mariadb, allowing easy inspection and modification of the database.

