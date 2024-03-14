ifeq (,$(COMPILER))
	COMPILER=gcc
endif

build:
	@$(COMPILER) login.c  -o login -fno-stack-protector > /dev/null 2>&1

execute:
	./login


run: build execute