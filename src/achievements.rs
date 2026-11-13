//! Stage 7: Achievements Registry & Engine

use crate::i18n::Translation;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Achievement {
    pub id: &'static str,
    pub icon: &'static str,
    pub xp_bonus: i64,
    pub category: &'static str,
}

impl Achievement {
    pub fn get_title(&self, t: &Translation) -> &'static str {
        match self.id {
            "first_spark" => t.ach_first_spark_title,
            "shadow_weaver" => t.ach_shadow_weaver_title,
            "tablet_bearer" => t.ach_tablet_bearer_title,
            "borrow_sage" => t.ach_borrow_sage_title,
            "slice_master" => t.ach_slice_master_title,
            "chapter1_conqueror" => t.ach_chapter1_conqueror_title,
            "grand_architect" => t.ach_grand_architect_title,
            "century_spark" => t.ach_century_spark_title,
            "high_paragon" => t.ach_high_paragon_title,
            "codex_scholar" => t.ach_codex_scholar_title,
            "consecrated_traveler" => t.ach_consecrated_traveler_title,
            "garden_founder" => t.ach_garden_founder_title,
            _ => "Таинственное достижение",
        }
    }

    pub fn get_desc(&self, t: &Translation) -> &'static str {
        match self.id {
            "first_spark" => t.ach_first_spark_desc,
            "shadow_weaver" => t.ach_shadow_weaver_desc,
            "tablet_bearer" => t.ach_tablet_bearer_desc,
            "borrow_sage" => t.ach_borrow_sage_desc,
            "slice_master" => t.ach_slice_master_desc,
            "chapter1_conqueror" => t.ach_chapter1_conqueror_desc,
            "grand_architect" => t.ach_grand_architect_desc,
            "century_spark" => t.ach_century_spark_desc,
            "high_paragon" => t.ach_high_paragon_desc,
            "codex_scholar" => t.ach_codex_scholar_desc,
            "consecrated_traveler" => t.ach_consecrated_traveler_desc,
            "garden_founder" => t.ach_garden_founder_desc,
            _ => "Сокрыто в глубинах святилища.",
        }
    }
}

pub const ALL_ACHIEVEMENTS: &[Achievement] = &[
    Achievement {
        id: "first_spark",
        icon: "⚡",
        xp_bonus: 25,
        category: "quest",
    },
    Achievement {
        id: "shadow_weaver",
        icon: "👤",
        xp_bonus: 30,
        category: "quest",
    },
    Achievement {
        id: "tablet_bearer",
        icon: "🏛️",
        xp_bonus: 35,
        category: "quest",
    },
    Achievement {
        id: "borrow_sage",
        icon: "🛡️",
        xp_bonus: 40,
        category: "quest",
    },
    Achievement {
        id: "slice_master",
        icon: "🔪",
        xp_bonus: 50,
        category: "quest",
    },
    Achievement {
        id: "chapter1_conqueror",
        icon: "📜",
        xp_bonus: 75,
        category: "chapter",
    },
    Achievement {
        id: "grand_architect",
        icon: "👑",
        xp_bonus: 150,
        category: "mastery",
    },
    Achievement {
        id: "century_spark",
        icon: "🌟",
        xp_bonus: 50,
        category: "progression",
    },
    Achievement {
        id: "high_paragon",
        icon: "💎",
        xp_bonus: 100,
        category: "progression",
    },
    Achievement {
        id: "codex_scholar",
        icon: "📖",
        xp_bonus: 30,
        category: "exploration",
    },
    Achievement {
        id: "consecrated_traveler",
        icon: "🦀",
        xp_bonus: 50,
        category: "social",
    },
    Achievement {
        id: "garden_founder",
        icon: "🏡",
        xp_bonus: 75,
        category: "social",
    },
];

pub fn find_achievement(id: &str) -> Option<&'static Achievement> {
    ALL_ACHIEVEMENTS.iter().find(|a| a.id == id)
}

/// Serialized achievement view for Tera templates
#[derive(Debug, Clone, Serialize)]
pub struct AchievementView {
    pub id: String,
    pub icon: String,
    pub title: String,
    pub desc: String,
    pub xp_bonus: i64,
    pub category: String,
    pub is_unlocked: bool,
    pub unlocked_at: Option<String>,
}
