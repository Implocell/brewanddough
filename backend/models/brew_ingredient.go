package models

import "gorm.io/gorm"

type BrewIngredient struct {
	gorm.Model
	BrewID       int  `json:"brewID"`
	Brew         Brew `json:"-"`
	IngredientID int  `json:"ingredientID"`
	Ingredient   Ingredient
	Amount       int `json:"amount"`
}
