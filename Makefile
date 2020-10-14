LDFLAGS = -L. -l:target/debug/libcoralenv.a -lrt -lpthread -ldl -lm

OBJS = main.o

all: main

%.o: %.c
	$(CC) -c -o $@ $<

main: $(OBJS)
	cargo build
	$(CC) -o main $^ $(LDFLAGS)

clean:
	$(RM) *~ *.o main
