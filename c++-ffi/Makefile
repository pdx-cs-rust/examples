CC = clang
CXX = clang++
RUSTC = rustc -C panic="abort"

OBJS = add1-c.o add1-rs.a demo.o

TARGETS = demo-c demo-rs

all: $(TARGETS)

demo-c: demo.o add1-c.o
	$(CXX) -o demo-c demo.o add1-c.o

demo-rs: demo.o add1-rs.a
	$(CXX) -o demo-rs demo.o add1-rs.a -lpthread -ldl

add1-c.o:
	$(CC) -c -o add1-c.o add1.c

add1-rs.a: add1.rs
	$(RUSTC) -o add1-rs.a add1.rs

clean:
	-rm -f $(TARGETS) $(OBJS)
