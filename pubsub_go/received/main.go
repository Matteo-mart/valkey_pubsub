package main

import (
	"context"
	"fmt"

	"github.com/redis/go-redis/v9"
)

func main() {

	ClearTerminal()

	ctx := context.Background()
	channelName := "local_channel"

	rdc := redis.NewClient(&redis.Options{
		// Addr: "redis://127.0.0.1:6379",
		Addr: "127.0.0.1:6379",
	})

	pubsub := rdc.Subscribe(ctx, channelName)
	defer pubsub.Close()

	fmt.Printf("\nConnecté à '%s'\n", channelName)

	channel := pubsub.Channel()

	for msg := range channel {
		fmt.Printf("Reçu sur %s: '%s'\n", msg.Channel, msg.Payload)

		if msg.Payload == "exit" {
			fmt.Println("\nFIN")
			break
		}
	}
}
