BUILDDIR := build/linux/
TOOL := build/linux/refunct-tas
LIB := build/linux/librtil.so

.PHONY: all
all: $(TOOL) $(LIB) lua

.PHONY: zip
zip: all
	cd build/linux && zip refunct-tas-linux.zip *

.PHONY: clippy
clippy:
	cd rtil && cargo clippy
	cd tool && cargo clippy

.PHONY: $(TOOL) # always execute cargo
$(TOOL): $(BUILDDIR)
	cd tool && cargo build
	cp tool/target/debug/refunct-tas $(TOOL)

.PHONY: $(LIB) # always execute cargo
$(LIB): $(BUILDDIR)
	cd rtil && rustup run nightly cargo build --release
	cp rtil/target/release/librtil.so $(LIB)

$(BUILDDIR):
	mkdir -p $(BUILDDIR)

.PHONY: lua
lua: $(BUILDDIR)
	bash -c 'cp tool/*.lua $(BUILDDIR)'
	cp tool/Config.toml $(BUILDDIR)
	sed -i "s/'v'/'w'/; s/'i'/'s'/; s/'a'/'d'/; s/'u'/'a'/" $(BUILDDIR)/Config.toml

.PHONY: clean
clean:
	$(RM) -r build/
	cd tool && cargo clean
	cd rtil && cargo clean
