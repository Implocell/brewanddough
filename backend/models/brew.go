package models

import "gorm.io/gorm"

type Brew struct {
	gorm.Model
	Name                   string            `json:"name"`
	OriginalGravity        int               `json:"originalGravity"`
	FinalGravity           int               `json:"finalGravity"`
	BrewType               int               `json:"brewType"`
	DryHops                bool              `json:"dryHops"`
	Carbonation            int               `json:"carbonation"`
	CarbonationIngredient  int               `json:"carbonationIngredient"`
	FermentationIngredient int               `json:"fermentationIngredient"`
	ABV                    int               `json:"abv"`
	DateStart              int               `json:"dateStart"`
	DateEnd                int               `json:"dateEnd"`
	Ingredients            []*BrewIngredient `json:"ingredients"`
}
