package brew

import (
	"errors"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/implocell/brewmaster/models"
)

type BrewService interface {
	Create(c *gin.Context)
	Get(c *gin.Context)
	GetAll(c *gin.Context)
	AddIngredient(c *gin.Context)
}

type brewService struct {
	repo Repository
}

func NewBrewService(repo Repository) brewService {
	return brewService{repo: repo}
}

func (s *brewService) Create(c *gin.Context) {
	var brewObject models.Brew
	if err := c.BindJSON(&brewObject); err != nil {
		c.AbortWithStatus(500)
		return
	}

	domainBrew, err := NewBrewFromObject(brewObject)

	if err != nil {
		c.AbortWithError(500, err)
	}

	brewObject = domainBrew.AsObject()

	_, err = s.repo.Create(c, &brewObject)
	if err != nil {
		c.AbortWithError(500, err)
		return
	}
	c.JSON(201, brewObject)
}

func (s *brewService) Get(c *gin.Context) {
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
	brew, err := s.repo.Get(c, uint(id))
	if err != nil {
		c.AbortWithError(500, err)
		return
	}

	c.JSON(200, brew)

}

func (s *brewService) GetAll(c *gin.Context) {
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

	brews, err := s.repo.GetAll(c, limit, offset)

	if err != nil {
		c.AbortWithError(500, err)
		return
	}

	c.JSON(200, brews)

}

func (s *brewService) AddIngredient(c *gin.Context) {
	var brewIngredient models.BrewIngredient

	if err := c.BindJSON(&brewIngredient); err != nil {
		c.AbortWithStatus(500)
		return
	}

	if err := s.repo.AddIngredient(c, brewIngredient); err != nil {
		c.AbortWithError(500, err)
		return
	}

	c.Status(200)
}
