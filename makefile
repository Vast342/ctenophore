EXE = ctenophore

ifeq ($(OS),Windows_NT)
    override EXE := $(EXE).exe
endif

# gotta make it detect pext eventually

all:
	cargo rustc --release -- -C target-cpu=native --emit link=$(EXE)

debug:
	cargo rustc -- -C target-cpu=native --emit link=$(EXE)

clean: 
	rm -rf $(EXE) target

run: all
	./$(EXE)

debug-run: debug
	./$(EXE)

# does nothing yet because we have no bench
bench: all
	./$(EXE) bench