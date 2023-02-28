package ingredient

import (
	"errors"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/implocell/brewmaster/models"
)

type IngredientService interface {
	Create(c *gin.Context)
	Get(c *gin.Context)
	GetAll(c *gin.Context)
}

type ingredientService struct {
	repo Repository
}

func NewIngredientService(repo Repository) ingredientService {
	return ingredientService{repo: repo}
}

func (s *ingredientService) Create(c *gin.Context) {
	var ingredientObject models.Ingredient
	if err := c.BindJSON(&ingredientObject); err != nil {
		c.AbortWithStatus(500)
		return
	}

	_, err := s.repo.Create(c, &ingredientObject)
	if err != nil {
		c.AbortWithError(500, err)
		return
	}
	c.JSON(201, ingredientObject)
}

func (s *ingredientService) Get(c *gin.Context) {
	idStr, ok := c.Params.Get("id")
	if !ok {
		c.AbortWithError(500, errors.New("failed to get id"))
		return
	}

	id, err := strconv.Atoi(idStr)
	if err != nil {
		c.AbortWithError(500, errors.New("failed to parse id to number"))
		return
	}
	ingredient, err := s.repo.Get(c, uint(id))
	if err != nil {
		c.AbortWithError(500, err)
		return
	}

	c.JSON(200, ingredient)

}

func (s *ingredientService) GetAll(c *gin.Context) {
	limitStr := c.DefaultQuery("limit", "100")

	limit, err := strconv.Atoi(limitStr)

	if err != nil {
		c.AbortWithError(500, err)
		return
	}

	offsetStr := c.DefaultQuery("offset", "0")

	offset, err := strconv.Atoi(offsetStr)

	if err != nil {
		c.AbortWithError(500, err)
		return
	}

	ingredients, err := s.repo.GetAll(c, limit, offset)

	if err != nil {
		c.AbortWithError(500, err)
		return
	}

	c.JSON(200, ingredients)

}
