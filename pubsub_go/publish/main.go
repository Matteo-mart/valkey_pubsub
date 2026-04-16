package main

import (
	"fmt"
)

func main() {
	ctx, rdc, channelName := setupRedis()

	fmt.Println("Connecté, Tapez 'exit' pour sortir:")

	for {
		var input string
		fmt.Scanln(&input)

		if input == "" {
			continue
		}

		rdc.Publish(ctx, channelName, input)

		if input == "exit" {
			fmt.Println("FIN")
			break
		}
	}
}
