package config

import (
	"fmt"
	"os"
	"github.com/joho/godotenv"
)

type Config struct {
	Port string
}

func LoadConfig() Config {
	err := godotenv.Load() 
	if err != nil {
		fmt.Println("Error loading .env file")
	}

	return Config{
		Port: os.Getenv("SERVER_PORT"),
	}
}
