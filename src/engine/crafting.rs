use crate::data::elements::Element;
use crate::data::items::{ItemQuality, Recipe};

pub struct CraftingContext {
    pub building_level: u32,
    pub building_element: Element,
    pub disciple_mind: Option<u32>,
}

pub struct CraftingResult {
    pub success: bool,
    pub critical: bool,
    pub output_quality: ItemQuality,
    pub output_amount: u32,
    pub materials_lost_fraction: f32,
    pub message: String,
}

/// Calculate the success chance for a recipe given the crafting context.
/// Returns a value between 5 and 95.
pub fn calculate_success_chance(recipe: &Recipe, ctx: &CraftingContext) -> u32 {
    let base = 100i32 - recipe.difficulty as i32;
    let building_bonus = (ctx.building_level.saturating_sub(1) * 5) as i32;
    let mind_bonus = ctx.disciple_mind.unwrap_or(0) as i32 / 10;
    let synergy_bonus = if let Some(ref affinity) = recipe.element_affinity {
        if *affinity != Element::None && *affinity == ctx.building_element {
            10
        } else {
            0
        }
    } else {
        0
    };

    let total = base + building_bonus + mind_bonus + synergy_bonus;
    total.clamp(5, 95) as u32
}

/// Perform a crafting attempt. Uses a simple RNG seed based on the provided random value (0-99).
pub fn calculate_craft(
    recipe: &Recipe,
    ctx: &CraftingContext,
    roll: u32, // 0-99 random roll
) -> CraftingResult {
    let success_chance = calculate_success_chance(recipe, ctx);
    let success = roll < success_chance;

    if !success {
        return CraftingResult {
            success: false,
            critical: false,
            output_quality: ItemQuality::Common,
            output_amount: 0,
            materials_lost_fraction: 0.5,
            message: format!(
                "Refinement of {} failed. Some offerings were lost to smoke.",
                recipe.name
            ),
        };
    }

    // Critical success check: success_chance / 5 threshold
    let crit_threshold = success_chance / 5;
    let critical = roll < crit_threshold;

    // Determine base quality from building level
    let mut quality_tier: u32 = 0; // Common
    if ctx.building_level >= 3 {
        // Chance for Superior (use second half of roll to vary)
        if roll % 3 == 0 {
            quality_tier = 2;
        } else if roll % 2 == 0 {
            quality_tier = 1;
        }
    } else if ctx.building_level >= 2 {
        if roll % 2 == 0 {
            quality_tier = 1;
        }
    }

    // Synergy bonus: chance for +1 tier
    if let Some(ref affinity) = recipe.element_affinity {
        if *affinity != Element::None && *affinity == ctx.building_element && roll % 4 == 0 {
            quality_tier += 1;
        }
    }

    // Critical: +1 tier
    if critical {
        quality_tier += 1;
    }

    let output_quality = ItemQuality::from_tier(quality_tier.min(4));
    let output_amount = if critical {
        recipe.output_amount + 1
    } else {
        recipe.output_amount
    };

    let quality_str = if output_quality != ItemQuality::Common {
        format!(" ({} quality)", output_quality)
    } else {
        String::new()
    };

    let crit_str = if critical { " Critical success!" } else { "" };

    CraftingResult {
        success: true,
        critical,
        output_quality,
        output_amount,
        materials_lost_fraction: 0.0,
        message: format!(
            "Refined {}x {}{}!{}",
            output_amount, recipe.name, quality_str, crit_str
        ),
    }
}
