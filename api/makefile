prog :=xnixperms

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)-$(extension)

all: build install

dockerup: 
	docker-compose up -d

dockerdown: 
	docker-compose down

migrateup:
  db-migrate up

run:
  cargo run
  
help:
	@echo "usage: make $(prog) [debug=1]"