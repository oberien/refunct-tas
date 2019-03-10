BUILDDIR := build/linux/
TOOL := build/linux/refunct-tas
LIB := build/linux/librtil.so

all: $(TOOL) $(LIB) lua

zip: all
	cd build/linux && zip refunct-tas-linux.zip *

clippy:
	cd rtil && cargo clippy
	cd tool && cargo clippy

$(TOOL): $(BUILDDIR)
	cd tool && cargo build
	cp tool/target/debug/refunct-tas $(TOOL)

$(LIB): $(BUILDDIR)
	cd rtil && rustup run nightly cargo build --release
	cp rtil/target/release/librtil.so $(LIB)

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

lua: $(BUILDDIR)
	bash -c 'cp tool/{crouch,ngg,prelude,printstats,rotation,setposition,setvelocity,teleportbuttons,turn,menu,record,keys,ui}.lua $(BUILDDIR)'

clean:
	$(RM) -r build/
	cd tool && cargo clean
	cd rtil && cargo clean

.PHONY: all clean $(TOOL) $(LIB) # always execute cargo and let it handle sources
