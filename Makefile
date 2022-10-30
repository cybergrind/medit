
venv: requirements.txt
	python3 -m venv venv
	./venv/bin/pip install -r requirements.txt


venv/bin/pip-compile: venv
	./venv/bin/pip install pip-tools

requirements.txt: requirements.in venv/bin/pip-compile
	./venv/bin/pip-compile requirements.in > requirements.txt
