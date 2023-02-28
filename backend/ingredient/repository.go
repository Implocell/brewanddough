package ingredient

import (
	"context"

	"github.com/implocell/brewmaster/models"
	"gorm.io/gorm"
)

type Repository interface {
	Create(ctx context.Context, ingredient *models.Ingredient) (uint, error)
	Get(ctx context.Context, id uint) (models.Ingredient, error)
	GetAll(ctx context.Context, limit int, offset int) ([]models.Ingredient, error)
}

type repository struct {
	db *gorm.DB
}

func NewRepository(db *gorm.DB) repository {
	return repository{db: db}
}

func (r *repository) Create(ctx context.Context, ingredient *models.Ingredient) (uint, error) {
	d := r.db.WithContext(ctx).Create(ingredient)

	if d.Error != nil {
		return 0, d.Error
	}

	return ingredient.ID, nil
}

func (r *repository) Get(ctx context.Context, id uint) (models.Ingredient, error) {

	var i models.Ingredient

	err := r.db.WithContext(ctx).First(&i, id).Error

	return i, err
}

func (r *repository) GetAll(ctx context.Context, limit int, offset int) ([]models.Ingredient, error) {
	var ingredients []models.Ingredient

	err := r.db.WithContext(ctx).Limit(limit).Offset(offset).Find(&ingredients).Error

	return ingredients, err
}
