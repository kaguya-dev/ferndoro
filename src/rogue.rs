use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::notification::Notifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnteTier {
    SafeBet1x,
    DoubleDown2x,
    AllInBoss3x,
    AllInBoss5x,
}

impl AnteTier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::SafeBet1x => "1x Safe Bet",
            Self::DoubleDown2x => "2x Double Down",
            Self::AllInBoss3x => "3x Boss Fight",
            Self::AllInBoss5x => "5x Nightmare Boss",
        }
    }

    pub fn multiplier(&self) -> f32 {
        match self {
            Self::SafeBet1x => 1.0,
            Self::DoubleDown2x => 2.0,
            Self::AllInBoss3x => 3.0,
            Self::AllInBoss5x => 5.0,
        }
    }

    pub fn max_pauses(&self) -> u32 {
        match self {
            Self::SafeBet1x => 2,
            Self::DoubleDown2x => 0,
            Self::AllInBoss3x => 0,
            Self::AllInBoss5x => 0,
        }
    }

    pub fn min_minutes(&self) -> u32 {
        match self {
            Self::SafeBet1x => 15,
            Self::DoubleDown2x => 25,
            Self::AllInBoss3x => 50,
            Self::AllInBoss5x => 60,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::SafeBet1x => "Standard focus session. Up to 2 pauses allowed. Base gold/XP rewards.",
            Self::DoubleDown2x => "2x Rewards. Zero pauses. Leaving app triggers 10s grace countdown before bust!",
            Self::AllInBoss3x => "3x Boss Fight (min 50m). Strict focus lock. Failure wipes unbanked pot. High loot!",
            Self::AllInBoss5x => "5x Nightmare Boss (min 60m). Maximum stakes. Enormous gold payout and rare relics.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FateModifier {
    None,
    // Boons
    TemporalFlux,   // +20% reward multiplier
    GoldenTouch,    // 2x gold payout
    SoulShield,     // +1 free pause forgiveness during Hard Mode
    Jackpot,        // Instant +75 Gold bonus
    // Curses
    BlindRun,       // Countdown masked (??:??) until <60s remain
    SuddenDeath,    // 60-second warmup focus lock before timer starts
    DoubleOrNothing,// Forces 2x Hard Mode, pays 3x if survived
}

impl FateModifier {
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "No Active Modifier",
            Self::TemporalFlux => "Temporal Flux (Boon)",
            Self::GoldenTouch => "Golden Touch (Boon)",
            Self::SoulShield => "Soul Shield (Boon)",
            Self::Jackpot => "Jackpot! (Boon)",
            Self::BlindRun => "Blind Run (Curse)",
            Self::SuddenDeath => "Sudden Death (Curse)",
            Self::DoubleOrNothing => "Double or Nothing (Curse)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::None => "Clear mind, no magical buffs or curses.",
            Self::TemporalFlux => "Next session grants +20% bonus Gold and XP.",
            Self::GoldenTouch => "Next session yields double (2x) Gold payout.",
            Self::SoulShield => "Grants 1 free pause forgiveness during Hard Mode/Boss Fights.",
            Self::Jackpot => "Fortune smiles! Instant +75 Gold deposited into unbanked pot.",
            Self::BlindRun => "The timer is concealed (??:??). Only sound signals session completion.",
            Self::SuddenDeath => "Requires a strict 60-second warmup focus lock before timer starts.",
            Self::DoubleOrNothing => "Forced 2x Hard Mode, but triples (3x) your payout if survived!",
        }
    }

    pub fn is_boon(&self) -> bool {
        matches!(self, Self::TemporalFlux | Self::GoldenTouch | Self::SoulShield | Self::Jackpot)
    }

    #[allow(dead_code)]
    pub fn is_curse(&self) -> bool {
        matches!(self, Self::BlindRun | Self::SuddenDeath | Self::DoubleOrNothing)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoguePhase {
    IdleAnte,       // Selecting task, duration, and ante difficulty
    WarmupLock,     // Sudden death 60s warmup
    FocusActive,    // Dungeon encounter in progress
    FocusFailed,    // Busted (lost unbanked pot)
    BreakDealer,    // 3-card fate draw micro-game (break timer runs in background)
    BreakRest,      // Calm pure rest state
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterClass {
    Chronomancer,
    BattleScholar,
    RuneAlchemist,
    ShadowScribe,
}

impl Default for CharacterClass {
    fn default() -> Self {
        Self::Chronomancer
    }
}

impl CharacterClass {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Chronomancer => "The Chronomancer",
            Self::BattleScholar => "The Battle Scholar",
            Self::RuneAlchemist => "The Rune Alchemist",
            Self::ShadowScribe => "The Shadow Scribe",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Chronomancer => "⏳",
            Self::BattleScholar => "⚔️",
            Self::RuneAlchemist => "🔮",
            Self::ShadowScribe => "📜",
        }
    }

    pub fn role_title(&self) -> &'static str {
        match self {
            Self::Chronomancer => "Master of Time & Flux",
            Self::BattleScholar => "High-Stakes Disciplinarian",
            Self::RuneAlchemist => "Transmuter of Knowledge & Fate",
            Self::ShadowScribe => "Silent Discipline Specialist",
        }
    }

    pub fn perk_description(&self) -> &'static str {
        match self {
            Self::Chronomancer => "+5s bonus grace window on focus break; +10% temporal XP.",
            Self::BattleScholar => "Starts runs with +1 Soul Shield on Floor 1; +20% Gold on Boss fights.",
            Self::RuneAlchemist => "+15% higher chance for Boon Fate cards; +25% XP from all completed floors.",
            Self::ShadowScribe => "+15 bonus Gold per completed floor; daily streak bonus doubled.",
        }
    }

    pub fn all() -> &'static [CharacterClass] {
        &[
            Self::Chronomancer,
            Self::BattleScholar,
            Self::RuneAlchemist,
            Self::ShadowScribe,
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RelicInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: u32,
    pub icon: &'static str,
}

pub const ALL_RELICS: &[RelicInfo] = &[
    RelicInfo {
        id: "loaded_dice",
        name: "Loaded Dice",
        desc: "Shifts fate card draw odds +15% towards Boons",
        cost: 150,
        icon: "🎲",
    },
    RelicInfo {
        id: "merchants_coin",
        name: "Merchant's Coin",
        desc: "+20% bonus Gold on all cleared floors",
        cost: 200,
        icon: "🪙",
    },
    RelicInfo {
        id: "hourglass_of_mercy",
        name: "Hourglass of Mercy",
        desc: "+5s extra grace period on window unfocus (15s total)",
        cost: 250,
        icon: "⏳",
    },
    RelicInfo {
        id: "aegis_of_will",
        name: "Aegis of Will",
        desc: "Start every dungeon run with +1 Soul Shield",
        cost: 300,
        icon: "🛡️",
    },
    RelicInfo {
        id: "crown_of_wisdom",
        name: "Crown of Wisdom",
        desc: "Grants +35% bonus XP from all dungeon encounters",
        cost: 320,
        icon: "👑",
    },
    RelicInfo {
        id: "phoenix_feather",
        name: "Phoenix Feather",
        desc: "Once per run: if you bust, preserves unbanked gold to Vault",
        cost: 400,
        icon: "🪶",
    },
    RelicInfo {
        id: "ring_of_clarity",
        name: "Ring of Clarity",
        desc: "Adds +5 bonus Gold for each floor cleared in active run",
        cost: 220,
        icon: "💍",
    },
    RelicInfo {
        id: "elixir_of_haste",
        name: "Elixir of Haste",
        desc: "Halves sanctuary rest duration and awards +25 XP",
        cost: 180,
        icon: "🧪",
    },
    RelicInfo {
        id: "tome_of_revelation",
        name: "Tome of Mystic Sight",
        desc: "Pre-reveals mystery cards in Floor Dealer break casino",
        cost: 260,
        icon: "📖",
    },
    RelicInfo {
        id: "cursed_chalice",
        name: "Cursed Chalice of Midas",
        desc: "Increases all Gold payouts by +50%",
        cost: 450,
        icon: "🏆",
    },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentRun {
    pub floor_number: u32,
    pub unbanked_gold: u32,
    pub current_multiplier: f32,
    pub active_modifier: FateModifier,
    pub soul_shields: u32,
    pub pauses_used: u32,
    pub floor_duration_mins: u32,
    pub selected_ante: AnteTier,
    pub selected_task: String,
    pub is_boss_floor: bool,

    #[serde(default)]
    pub sessions_in_run: u32,
}

impl Default for CurrentRun {
    fn default() -> Self {
        Self {
            floor_number: 1,
            unbanked_gold: 0,
            current_multiplier: 1.0,
            active_modifier: FateModifier::None,
            soul_shields: 0,
            pauses_used: 0,
            floor_duration_mins: 25,
            selected_ante: AnteTier::SafeBet1x,
            selected_task: "Floor 1 • Session #1 (Standard Focus Encounter)".to_string(),
            is_boss_floor: false,
            sessions_in_run: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVault {
    pub level: u32,
    pub current_xp: u32,
    pub vault_gold: u32,
    pub total_dungeon_minutes: u32,
    pub highest_floor: u32,
    pub total_runs_cashed: u32,
    pub unlocked_relics: HashSet<String>,
    pub equipped_title: String,

    #[serde(default)]
    pub selected_class: CharacterClass,

    #[serde(default)]
    pub total_sessions_completed: u32,
}

impl Default for UserVault {
    fn default() -> Self {
        Self {
            level: 1,
            current_xp: 0,
            vault_gold: 60,
            total_dungeon_minutes: 0,
            highest_floor: 1,
            total_runs_cashed: 0,
            unlocked_relics: HashSet::new(),
            equipped_title: "Novice Scribe".to_string(),
            selected_class: CharacterClass::Chronomancer,
            total_sessions_completed: 0,
        }
    }
}

impl UserVault {
    pub fn xp_threshold(&self) -> u32 {
        (self.level * 100).max(100)
    }

    pub fn xp_progress_fraction(&self) -> f32 {
        let thresh = self.xp_threshold();
        (self.current_xp as f32 / thresh as f32).clamp(0.0, 1.0)
    }

    /// Adds XP and returns Some(new_level) if leveled up
    pub fn add_xp(&mut self, xp: u32) -> Option<u32> {
        self.current_xp += xp;
        let mut leveled_up = false;

        while self.current_xp >= self.xp_threshold() {
            self.current_xp -= self.xp_threshold();
            self.level += 1;
            leveled_up = true;
            self.update_title();
        }

        if leveled_up {
            Some(self.level)
        } else {
            None
        }
    }

    pub fn update_title(&mut self) {
        self.equipped_title = match self.level {
            1..=2 => "Apprentice Scribe".to_string(),
            3..=4 => "Dungeon Scholar".to_string(),
            5..=7 => "Chrono-Adept".to_string(),
            8..=11 => "Arcane Strategist".to_string(),
            _ => "Grand Chronomancer".to_string(),
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RogueDungeonState {
    pub phase: RoguePhase,
    pub current_run: CurrentRun,
    pub vault: UserVault,
    
    // In-encounter live timers
    pub remaining_focus_seconds: f32,
    pub remaining_break_seconds: f32,
    pub remaining_warmup_seconds: f32,
    pub grace_period_seconds: f32,
    pub is_grace_active: bool,

    // Dealer cards for BreakDealer state
    pub dealer_cards: Vec<FateModifier>,
    pub card_drawn: Option<FateModifier>,
    pub dealer_resolved: bool,
}

impl Default for RogueDungeonState {
    fn default() -> Self {
        Self {
            phase: RoguePhase::IdleAnte,
            current_run: CurrentRun::default(),
            vault: UserVault::default(),
            remaining_focus_seconds: 25.0 * 60.0,
            remaining_break_seconds: 5.0 * 60.0,
            remaining_warmup_seconds: 60.0,
            grace_period_seconds: 10.0,
            is_grace_active: false,
            dealer_cards: Vec::new(),
            card_drawn: None,
            dealer_resolved: false,
        }
    }
}

impl RogueDungeonState {
    pub fn max_grace_period(&self) -> f32 {
        let mut grace = 10.0;
        if self.vault.unlocked_relics.contains("hourglass_of_mercy") {
            grace += 5.0;
        }
        if self.vault.selected_class == CharacterClass::Chronomancer {
            grace += 5.0;
        }
        grace
    }

    pub fn auto_encounter_title(&self, active_course: Option<&str>) -> String {
        let session_num = self.vault.total_sessions_completed + 1;
        let floor = self.current_run.floor_number;

        let tier_name = match self.current_run.selected_ante {
            AnteTier::SafeBet1x => "Standard Focus Encounter",
            AnteTier::DoubleDown2x => "High-Stakes Trial",
            AnteTier::AllInBoss3x => "Guardian Boss Raid",
            AnteTier::AllInBoss5x => "Nightmare Overlord Battle",
        };

        if let Some(course) = active_course {
            if !course.trim().is_empty() {
                return format!("Floor {} • Session #{} ({} • {})", floor, session_num, course, tier_name);
            }
        }

        format!("Floor {} • Session #{} ({})", floor, session_num, tier_name)
    }

    pub fn calculate_floor_payout(&self) -> u32 {
        let base_gold = (self.current_run.floor_duration_mins as f32 * 0.5).ceil() as u32;
        let ante_mult = self.current_run.selected_ante.multiplier();
        let run_mult = self.current_run.current_multiplier;

        let mut mod_mult = 1.0_f32;
        if self.current_run.active_modifier == FateModifier::TemporalFlux {
            mod_mult *= 1.2;
        } else if self.current_run.active_modifier == FateModifier::GoldenTouch {
            mod_mult *= 2.0;
        } else if self.current_run.active_modifier == FateModifier::DoubleOrNothing {
            mod_mult *= 3.0;
        }

        let mut relic_mult = 1.0_f32;
        if self.vault.unlocked_relics.contains("merchants_coin") {
            relic_mult += 0.20;
        }
        if self.vault.unlocked_relics.contains("cursed_chalice") {
            relic_mult += 0.50;
        }
        if self.vault.selected_class == CharacterClass::BattleScholar && self.current_run.selected_ante != AnteTier::SafeBet1x {
            relic_mult += 0.20;
        }

        let streak_bonus = if self.vault.unlocked_relics.contains("ring_of_clarity") {
            self.current_run.sessions_in_run * 5
        } else {
            0
        };

        let class_flat = if self.vault.selected_class == CharacterClass::ShadowScribe { 15 } else { 0 };

        (base_gold as f32 * ante_mult * run_mult * mod_mult * relic_mult).round() as u32 + streak_bonus + class_flat
    }

    pub fn calculate_floor_xp(&self) -> u32 {
        let base_xp = self.current_run.floor_duration_mins * 2;
        let ante_mult = self.current_run.selected_ante.multiplier();

        let mut xp_mult = 1.0_f32;
        if self.vault.unlocked_relics.contains("crown_of_wisdom") {
            xp_mult += 0.35;
        }
        if self.vault.selected_class == CharacterClass::RuneAlchemist {
            xp_mult += 0.25;
        }
        if self.vault.selected_class == CharacterClass::Chronomancer {
            xp_mult += 0.10;
        }

        (base_xp as f32 * ante_mult * xp_mult).round() as u32
    }

    pub fn start_encounter(&mut self, task_name: &str) {
        self.current_run.selected_task = task_name.to_string();

        if self.current_run.active_modifier == FateModifier::DoubleOrNothing {
            if self.current_run.selected_ante == AnteTier::SafeBet1x {
                self.current_run.selected_ante = AnteTier::DoubleDown2x;
            }
        }

        if self.current_run.floor_number == 1 && self.current_run.soul_shields == 0 {
            let mut shields = 0;
            if self.vault.unlocked_relics.contains("aegis_of_will") {
                shields += 1;
            }
            if self.vault.selected_class == CharacterClass::BattleScholar {
                shields += 1;
            }
            self.current_run.soul_shields = shields;
        }

        self.current_run.pauses_used = 0;
        self.grace_period_seconds = self.max_grace_period();
        self.is_grace_active = false;

        if self.current_run.active_modifier == FateModifier::SuddenDeath {
            self.phase = RoguePhase::WarmupLock;
            self.remaining_warmup_seconds = 60.0;
        } else {
            self.phase = RoguePhase::FocusActive;
            self.remaining_focus_seconds = (self.current_run.floor_duration_mins * 60) as f32;
        }
    }

    pub fn pause_encounter(&mut self) -> bool {
        if self.phase != RoguePhase::FocusActive {
            return false;
        }

        let max_pauses = self.current_run.selected_ante.max_pauses();
        if self.current_run.pauses_used < max_pauses {
            self.current_run.pauses_used += 1;
            true
        } else if self.current_run.soul_shields > 0 {
            self.current_run.soul_shields -= 1;
            Notifier::notify("FernDoro Dungeon", "Soul Shield absorbed the pause! Penalty avoided.");
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self, dt: f32, window_focused: bool) -> Option<FloorCompletedEvent> {
        match self.phase {
            RoguePhase::WarmupLock => {
                self.remaining_warmup_seconds -= dt;
                if self.remaining_warmup_seconds <= 0.0 {
                    self.phase = RoguePhase::FocusActive;
                    self.remaining_focus_seconds = (self.current_run.floor_duration_mins * 60) as f32;
                    Notifier::notify("FernDoro Dungeon", "Warmup complete! The encounter has begun.");
                }
            }
            RoguePhase::FocusActive => {
                let is_strict_mode = self.current_run.selected_ante != AnteTier::SafeBet1x;
                if is_strict_mode {
                    if !window_focused {
                        self.is_grace_active = true;
                        self.grace_period_seconds -= dt;
                        if self.grace_period_seconds <= 0.0 {
                            self.trigger_bust();
                            return None;
                        }
                    } else {
                        self.is_grace_active = false;
                        self.grace_period_seconds = (self.grace_period_seconds + dt * 2.0).min(self.max_grace_period());
                    }
                }

                self.remaining_focus_seconds -= dt;

                if self.remaining_focus_seconds <= 0.0 {
                    self.remaining_focus_seconds = 0.0;
                    return Some(self.complete_floor());
                }
            }
            RoguePhase::BreakDealer | RoguePhase::BreakRest => {
                self.remaining_break_seconds -= dt;
                if self.remaining_break_seconds <= 0.0 {
                    self.remaining_break_seconds = 0.0;
                    Notifier::notify("Sanctuary Complete", "Your guaranteed rest is over. Ready for the next floor?");
                }
            }
            _ => {}
        }

        None
    }

    pub fn trigger_bust(&mut self) {
        if self.vault.unlocked_relics.contains("phoenix_feather") && self.current_run.unbanked_gold > 0 {
            let saved = self.current_run.unbanked_gold;
            self.vault.vault_gold += saved;
            self.current_run.unbanked_gold = 0;
            Notifier::notify(
                "🪶 Phoenix Feather Saved You!",
                &format!("The Phoenix Feather preserved your {} Gold and safely deposited it to Vault!", saved),
            );
        } else {
            self.current_run.unbanked_gold = 0;
            Notifier::notify(
                "☠️ DUNGEON BUST!",
                "You lost focus in a high-stakes encounter! Your unbanked pot has been lost to the void.",
            );
        }

        self.phase = RoguePhase::FocusFailed;
        self.current_run.current_multiplier = 1.0;
        self.is_grace_active = false;
    }

    pub fn complete_floor(&mut self) -> FloorCompletedEvent {
        let earned_gold = self.calculate_floor_payout();
        let earned_xp = self.calculate_floor_xp();

        self.current_run.unbanked_gold += earned_gold;
        self.vault.total_dungeon_minutes += self.current_run.floor_duration_mins;
        self.vault.total_sessions_completed += 1;
        self.current_run.sessions_in_run += 1;

        let leveled_up = self.vault.add_xp(earned_xp);

        if self.current_run.floor_number > self.vault.highest_floor {
            self.vault.highest_floor = self.current_run.floor_number;
        }

        self.current_run.current_multiplier += 0.25;
        self.current_run.active_modifier = FateModifier::None;

        let break_mins = if self.current_run.is_boss_floor { 10 } else { 5 };
        let mut final_break_secs = (break_mins * 60) as f32;
        if self.vault.unlocked_relics.contains("elixir_of_haste") {
            final_break_secs *= 0.5;
            self.vault.add_xp(25);
        }
        self.remaining_break_seconds = final_break_secs;

        self.populate_dealer_cards();
        self.card_drawn = None;
        self.dealer_resolved = false;
        self.phase = RoguePhase::BreakDealer;

        Notifier::notify(
            "⚔️ Floor Cleared!",
            &format!("Floor {} conquered! +{} Gold in Pot • +{} XP. Enter Sanctuary.", self.current_run.floor_number, earned_gold, earned_xp),
        );

        FloorCompletedEvent {
            floor: self.current_run.floor_number,
            gold_earned: earned_gold,
            xp_earned: earned_xp,
            task_name: self.current_run.selected_task.clone(),
            duration_minutes: self.current_run.floor_duration_mins,
            leveled_up,
        }
    }

    pub fn populate_dealer_cards(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_micros();

        let has_loaded_dice = self.vault.unlocked_relics.contains("loaded_dice");
        let is_rune_alchemist = self.vault.selected_class == CharacterClass::RuneAlchemist;
        
        let boons = [
            FateModifier::TemporalFlux,
            FateModifier::GoldenTouch,
            FateModifier::SoulShield,
            FateModifier::Jackpot,
        ];
        let curses = [
            FateModifier::BlindRun,
            FateModifier::SuddenDeath,
            FateModifier::DoubleOrNothing,
        ];

        let mut cards = Vec::with_capacity(3);
        for i in 0..3 {
            let roll = ((seed + i * 7919) % 100) as u32;
            let mut boon_threshold = 55;
            if has_loaded_dice {
                boon_threshold += 15;
            }
            if is_rune_alchemist {
                boon_threshold += 15;
            }

            if roll < boon_threshold {
                let idx = ((seed / (i + 1) as u128) % boons.len() as u128) as usize;
                cards.push(boons[idx]);
            } else {
                let idx = ((seed / (i + 1) as u128) % curses.len() as u128) as usize;
                cards.push(curses[idx]);
            }
        }

        self.dealer_cards = cards;
    }

    pub fn draw_card(&mut self, card_index: usize) -> Option<FateModifier> {
        if card_index < self.dealer_cards.len() && !self.dealer_resolved {
            let drawn = self.dealer_cards[card_index];
            self.card_drawn = Some(drawn);
            self.dealer_resolved = true;

            match drawn {
                FateModifier::Jackpot => {
                    self.current_run.unbanked_gold += 75;
                }
                FateModifier::SoulShield => {
                    self.current_run.soul_shields += 1;
                }
                _ => {
                    self.current_run.active_modifier = drawn;
                }
            }

            return Some(drawn);
        }
        None
    }

    pub fn proceed_to_next_floor(&mut self) {
        self.current_run.floor_number += 1;
        self.phase = RoguePhase::IdleAnte;
    }

    pub fn cash_out(&mut self) -> u32 {
        let gold_to_bank = self.current_run.unbanked_gold;
        self.vault.vault_gold += gold_to_bank;
        self.vault.total_runs_cashed += 1;

        self.current_run = CurrentRun::default();
        self.phase = RoguePhase::IdleAnte;

        Notifier::notify(
            "💰 Cashed Out & Banked!",
            &format!("Successfully stored {} Gold safely in your Vault!", gold_to_bank),
        );

        gold_to_bank
    }

    pub fn buy_relic(&mut self, relic_id: &str, cost: u32) -> bool {
        if self.vault.vault_gold >= cost && !self.vault.unlocked_relics.contains(relic_id) {
            self.vault.vault_gold -= cost;
            self.vault.unlocked_relics.insert(relic_id.to_string());
            true
        } else {
            false
        }
    }
}

pub struct FloorCompletedEvent {
    pub floor: u32,
    pub gold_earned: u32,
    pub xp_earned: u32,
    pub task_name: String,
    pub duration_minutes: u32,
    pub leveled_up: Option<u32>,
}
