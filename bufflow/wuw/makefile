ifeq (,$(COMPILER))
	COMPILER=rustc
endif

build:
	@$(COMPILER) login.rs 

execute:
	./login


run: build execute