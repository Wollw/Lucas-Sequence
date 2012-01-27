TARGET	= lucas

CRATE     = main.rc

RUSTC	=	rustc
RUSTFLAGS =

all: build

build: 
	$(RUSTC) $(RUSTFLAGS) src/$(CRATE) -o $(TARGET)

clean:
	rm -rf $(TARGET)
