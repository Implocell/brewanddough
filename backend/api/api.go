package api

import (
	"github.com/gin-gonic/gin"
	"github.com/implocell/brewmaster/brew"
	"github.com/implocell/brewmaster/ingredient"
	"github.com/implocell/brewmaster/services"
)

func NewApiRouter(r *gin.Engine, services services.Services) error {
	api := r.Group("/api/v1")
	brewRouter(api, services.BrewService)
	ingredientRouter(api, services.IngredientService)
	return nil
}

func brewRouter(r *gin.RouterGroup, service brew.BrewService) error {
	r.POST("/brew", service.Create)
	r.GET("/brew/:id", service.Get)
	r.GET("/brews", service.GetAll)
	r.POST("/brew/addingredient", service.AddIngredient)

	return nil
}

func ingredientRouter(r *gin.RouterGroup, service ingredient.IngredientService) error {
	r.POST("/ingredient", service.Create)
	r.GET("/ingredient/:id", service.Get)
	r.GET("/ingredients", service.GetAll)

	return nil
}
