use crate::model::value_object::brew::BrewObject;

pub enum BrewType {
    Extract = 0,
    AllGrain = 1,
}

pub enum CarbonationType {
    Bottle = 0,
    Keg = 1,
}

pub enum CarbonationIngredient {
    Sugar = 0,
    CO2 = 1,
}

pub enum FermentationIngredient {
    Dextrose = 0,
    Malt = 1,
}

pub struct Brew {
    pub name: String,
    pub original_gravity: Option<i32>,
    pub final_gravity: Option<i32>,
    pub brew_type: BrewType,
    pub dry_hops: bool,
    pub carbonation: Option<CarbonationType>,
    pub carbonation_ingredient: Option<CarbonationIngredient>,
    pub fermentation_ingredient: Option<FermentationIngredient>,
    pub abv: Option<i32>,
    pub date_start: Option<i64>,
    pub date_end: Option<i64>,
}

impl Brew {
    pub fn from_object(obj: &BrewObject) -> Self {
        let brew_type = match obj.brew_type {
            0 => BrewType::Extract,
            1 => BrewType::AllGrain,
            _ => BrewType::Extract,
        };

        let carbonation = match obj.carbonation {
            Some(c) => match c {
                0 => Some(CarbonationType::Bottle),
                1 => Some(CarbonationType::Keg),
                _ => Some(CarbonationType::Bottle),
            },
            None => None,
        };

        let carbonation_ingredient = match obj.carbonation_ingredient {
            Some(c) => match c {
                0 => Some(CarbonationIngredient::Sugar),
                1 => Some(CarbonationIngredient::CO2),
                _ => Some(CarbonationIngredient::Sugar),
            },
            None => None,
        };

        let fermentation_ingredient = match obj.fermentation_ingredient {
            Some(f) => match f {
                0 => Some(FermentationIngredient::Dextrose),
                1 => Some(FermentationIngredient::Malt),
                _ => Some(FermentationIngredient::Dextrose),
            },
            None => None,
        };

        Brew {
            name: obj.name.to_owned(),
            original_gravity: obj.original_gravity,
            final_gravity: obj.final_gravity,
            brew_type,
            dry_hops: obj.dry_hops,
            carbonation,
            carbonation_ingredient,
            fermentation_ingredient,
            abv: obj.abv,
            date_start: obj.date_start,
            date_end: obj.date_end,
        }
    }

    pub fn to_object(&self) -> BrewObject {
        let brew_type = match self.brew_type {
            BrewType::Extract => 0,
            BrewType::AllGrain => 1,
        };

        let carbonation = match &self.carbonation {
            Some(c) => match c {
                CarbonationType::Bottle => Some(0),
                CarbonationType::Keg => Some(1),
            },
            None => None,
        };

        let carbonation_ingredient = match &self.carbonation_ingredient {
            Some(c) => match c {
                CarbonationIngredient::Sugar => Some(0),
                CarbonationIngredient::CO2 => Some(1),
            },
            None => None,
        };

        let fermentation_ingredient = match &self.fermentation_ingredient {
            Some(f) => match f {
                FermentationIngredient::Dextrose => Some(0),
                FermentationIngredient::Malt => Some(1),
            },
            None => None,
        };

        BrewObject {
            brew_id: String::default(),
            name: self.name.to_owned(),
            original_gravity: self.original_gravity,
            final_gravity: self.final_gravity,
            brew_type,
            dry_hops: self.dry_hops,
            carbonation,
            carbonation_ingredient,
            fermentation_ingredient,
            abv: self.abv,
            date_start: self.date_start,
            date_end: self.date_end,
            created_at: i64::default(),
            updated_at: i64::default(),
        }
    }
}
