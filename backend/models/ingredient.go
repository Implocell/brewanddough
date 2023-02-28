package models

import "gorm.io/gorm"

type Ingredient struct {
	gorm.Model
	Name           string            `json:"name"`
	Unit           string            `json:"unit"`
	IngredientType string            `json:"type"`
	Brews          []*BrewIngredient `json:"ingredients"`
}
