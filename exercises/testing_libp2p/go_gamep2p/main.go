package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	libp2p "github.com/libp2p/go-libp2p"
)

func main() {
	ctx := context.Background()

	host, err := libp2p.New(ctx)
	if err != nil {
		panic(err)
	}
	defer host.Close()

	fmt.Println(host.Addrs())

	sigCh := make(chan os.Signal)
	signal.Notify(sigCh, syscall.SIGKILL, syscall.SIGINT)
	<-sigCh
}
