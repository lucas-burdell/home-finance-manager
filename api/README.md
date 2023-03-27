# setup

build requirements: rust, cargo, docker, make, node, db-migrate (from npm)

Make sure Docker is installed and running. There are two docker containers: MySQL (sql port 3306) and Adminer (http port 8090). Adminer is an admin portal for SQL DBs.

Copy `.env.example` file and rename the copy to `.env`, change any variables as needed

Run in this order:
`make dockerup`
`make migrateup`
`make run`

# migrations
create a new migration:

`db-migrate create MY_FILENAME --sql-file`

then edit the files created
