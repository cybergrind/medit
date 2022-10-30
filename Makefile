
VENV_DIR = $(shell pwd)/venv

venv:
	python3 -m venv venv
	./venv/bin/pip install -r requirements.txt
	touch venv


venv/bin/pip-compile:
	./venv/bin/pip install pip-tools

requirements.txt: requirements.in
	./venv/bin/pip-compile requirements.in > requirements.txt

compile-dev: venv
	@export VIRTUAL_ENV=$(VENV_DIR) && \
	export PATH=$(PATH):$(VENV_DIR)/bin && \
	cd medit_rs && maturin develop

test: compile-dev
	./venv/bin/pytest

.PHONY: compile-dev test

