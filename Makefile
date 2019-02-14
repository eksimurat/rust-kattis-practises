.PHONY: list start stop status gitPush apache ubuntu
.DEFAULT_GOAL := start

COMPOSE = docker-compose -p $(PROJECT)
PROJECT ?= rust-workspace

list:
	@(grep -oe '^[[:lower:][:digit:]_\-]\{1,\}' Makefile | uniq)

start:
	$(COMPOSE) run -p 4444:4444 -p 8000:8000 --rm rust bash

stop:
	$(COMPOSE) stop

status:
	$(COMPOSE) ps

gitPush:
	@echo "Git Push Update Manager - Murat Eksi"
	git add .
	git commit -m "update"
	git push



