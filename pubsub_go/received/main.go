package main

import (
	"fmt"
)

func main() {
	ctx, rdc, channelName := setupRedis()

	fmt.Printf("\nConnecté à '%s'\n", channelName)

	pubsub := rdc.Subscribe(ctx, channelName)
	defer pubsub.Close()

	channel := pubsub.Channel()

	for msg := range channel {
		fmt.Printf("Reçu sur %s: '%s'\n", msg.Channel, msg.Payload)

		if msg.Payload == "exit" {
			fmt.Println("\nFIN")
			break
		}
	}

}
