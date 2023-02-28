package main

import (
	"io"
	"log"
	"net/http"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/implocell/brewmaster/api"
	"github.com/implocell/brewmaster/models"
	"github.com/implocell/brewmaster/services"
	"github.com/joho/godotenv"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
)

func main() {

	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	db, err := gorm.Open(postgres.Open(os.Getenv("DATABASE_URL")))
	if err != nil {
		log.Fatal(err)
	}

	db.AutoMigrate(models.Brew{}, models.Ingredient{}, models.BrewIngredient{})

	services := services.NewServices(db)

	gin.DisableConsoleColor()

	f, err := os.Create("server.log")
	if err != nil {
		log.Fatal(err)
	}

	gin.DefaultWriter = io.MultiWriter(f, os.Stdout)

	r := gin.Default()

	if err := api.NewApiRouter(r, services); err != nil {
		log.Fatal(err)
	}

	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})

	r.Run(":7050")
}
