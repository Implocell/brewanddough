package brew

import (
	"context"

	"github.com/implocell/brewmaster/models"
	"gorm.io/gorm"
)

type Repository interface {
	Create(ctx context.Context, brew *models.Brew) (uint, error)
	Get(ctx context.Context, id uint) (models.Brew, error)
	GetAll(ctx context.Context, limit int, offset int) ([]models.Brew, error)
	AddIngredient(ctx context.Context, brewIngredient models.BrewIngredient) error
}

type repository struct {
	db *gorm.DB
}

func NewRepository(db *gorm.DB) repository {
	return repository{db: db}
}

func (r *repository) Create(ctx context.Context, brew *models.Brew) (uint, error) {
	d := r.db.WithContext(ctx).Create(brew)

	if d.Error != nil {
		return 0, d.Error
	}

	return brew.ID, nil
}

func (r *repository) Get(ctx context.Context, id uint) (models.Brew, error) {
	var b models.Brew
	err := r.db.WithContext(ctx).Preload("Ingredients.Ingredient").Find(&b, id).Error
	// r.db.WithContext(ctx).First(&b, id).Error

	return b, err
}

func (r *repository) GetAll(ctx context.Context, limit int, offset int) ([]models.Brew, error) {
	var brews []models.Brew

	err := r.db.WithContext(ctx).Limit(limit).Offset(offset).Find(&brews).Error

	return brews, err
}

func (r *repository) AddIngredient(ctx context.Context, brewIngredient models.BrewIngredient) error {

	return r.db.WithContext(ctx).Create(&brewIngredient).Error
}
