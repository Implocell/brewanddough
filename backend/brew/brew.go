package brew

import (
	"github.com/implocell/brewmaster/models"
)

type brew struct {
	Name                   string
	OriginalGravity        int
	FinalGravity           int
	BrewType               int
	DryHops                bool
	Carbonation            int
	CarbonationIngredient  int
	FermentationIngredient int
	ABV                    int
	DateStart              int
	DateEnd                int
}

func NewBrewFromObject(obj models.Brew) (brew, error) {
	b := brew{
		Name:                   obj.Name,
		OriginalGravity:        obj.OriginalGravity,
		FinalGravity:           obj.FinalGravity,
		BrewType:               obj.BrewType,
		DryHops:                obj.DryHops,
		Carbonation:            obj.Carbonation,
		CarbonationIngredient:  obj.CarbonationIngredient,
		FermentationIngredient: obj.FermentationIngredient,
		ABV:                    obj.ABV,
		DateStart:              obj.DateStart,
		DateEnd:                obj.DateEnd,
	}

	if b.OriginalGravity != 0 && b.FinalGravity != 0 {
		b.calculateABV()
	}

	return b, nil
}

func (b *brew) calculateABV() {
	b.ABV = 100
}

func (b *brew) AsObject() models.Brew {
	return models.Brew{
		Name:                   b.Name,
		OriginalGravity:        b.OriginalGravity,
		FinalGravity:           b.FinalGravity,
		BrewType:               b.BrewType,
		DryHops:                b.DryHops,
		Carbonation:            b.Carbonation,
		CarbonationIngredient:  b.CarbonationIngredient,
		FermentationIngredient: b.FermentationIngredient,
		ABV:                    b.ABV,
		DateStart:              b.DateStart,
		DateEnd:                b.DateEnd,
	}
}
