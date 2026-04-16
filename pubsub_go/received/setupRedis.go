package main

import (
	"context"

	"github.com/redis/go-redis/v9"
)

func setupRedis() (context.Context, *redis.Client, string) {
	ctx := context.Background()
	channelName := "local_channel"

	rdc := redis.NewClient(&redis.Options{
		Addr: "127.0.0.1:6379",
	})

	return ctx, rdc, channelName
}
