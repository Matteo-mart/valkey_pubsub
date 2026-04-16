package main

import (
	"context"
	"fmt"

	"github.com/redis/go-redis/v9"
)

func main() {

	ClearTerminal()

	ctx := context.Background()
	rdc := redis.NewClient(&redis.Options{
		// Addr: "redis://127.0.0.1:6379",
		Addr: "127.0.0.1:6379",
	})

	fmt.Println("Connecté, Tapez 'exit' pour sortir:")

	for {
		var input string
		fmt.Scanln(&input)

		if input == "" {
			continue
		}

		rdc.Publish(ctx, "local_channel", input)

		if input == "exit" {
			fmt.Println("FIN")
			break
		}
	}
}
