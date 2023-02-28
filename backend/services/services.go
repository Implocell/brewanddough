package services

import (
	"github.com/implocell/brewmaster/brew"
	"github.com/implocell/brewmaster/ingredient"
	"gorm.io/gorm"
)

type Services struct {
	BrewService       brew.BrewService
	IngredientService ingredient.IngredientService
}

func NewServices(db *gorm.DB) Services {
	brewRepo := brew.NewRepository(db)
	brewService := brew.NewBrewService(&brewRepo)

	ingredientRepo := ingredient.NewRepository(db)
	ingredientService := ingredient.NewIngredientService(&ingredientRepo)

	return Services{
		BrewService:       &brewService,
		IngredientService: &ingredientService,
	}
}
