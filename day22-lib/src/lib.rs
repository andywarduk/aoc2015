// Parameters from input file
const BOSS_HIT_POINTS: u16 = 71;
const BOSS_DAMAGE: u16 = 10;

const PLAYER_HIT_POINTS: u16 = 50;
const PLAYER_MANA: u16 = 500;

const MISSILE_COST: u16 = 53;
const MISSILE_DAMAGE: u16 = 4;

const DRAIN_COST: u16 = 73;
const DRAIN_DAMAGE: u16 = 2;
const DRAIN_HEAL: u16 = 2;

const SHIELD_COST: u16 = 113;
const SHIELD_ARMOR: u8 = 7;
const SHIELD_TIME: u8 = 6;

const POISON_COST: u16 = 173;
const POISON_DAMAGE: u16 = 3;
const POISON_TIME: u8 = 6;

const RECHARGE_COST: u16 = 229;
const RECHARGE_MANA: u16 = 101;
const RECHARGE_TIME: u8 = 5;

pub enum Difficulty {
    Normal,
    Hard
}

#[derive(Clone, Debug)]
pub enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge
}

#[derive(Clone)]
struct PlayerState {
    hit_points: u16,
    mana: u16,
    armor: u8,
    shield_timer: u8,
    poison_timer: u8,
    recharge_timer: u8,
    spend: u16,
    spells_cast: Vec<Spell>
}

impl PlayerState {
    fn spend(&mut self, mana: u16) {
        self.mana -= mana;
        self.spend += mana;
    }

    fn casted(&mut self, spell: Spell) {
        self.spells_cast.push(spell);
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            hit_points: PLAYER_HIT_POINTS,
            mana: PLAYER_MANA,
            armor: 0,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            spend: 0,
            spells_cast: Vec::new()
        }
    }
}

#[derive(Clone)]
struct BossState {
    hit_points: u16
}

impl Default for BossState {
    fn default() -> Self {
        Self {
            hit_points: BOSS_HIT_POINTS
        }
    }
}

pub struct Result {
    pub min_spend: u16,
    pub min_spells: Vec<Spell>,
    pub boss_wins: u32,
    pub player_wins: u32
}

impl Default for Result {
    fn default() -> Self {
        Self {
            min_spend: u16::MAX,
            min_spells: Vec::new(),
            boss_wins: 0,
            player_wins: 0
        }
    }
}

pub fn play(difficulty: Difficulty) -> Result {
    let mut player: PlayerState = Default::default();
    let mut boss: BossState = Default::default();
    let mut result: Result = Default::default();

    player_turn(&difficulty, &mut result, &mut player, &mut boss);

    result
}

fn player_turn(difficulty: &Difficulty, result: &mut Result, player: &mut PlayerState, boss: &mut BossState) {
    loop {
        // Player's turn
        match difficulty {
            Difficulty::Hard => {
                player.hit_points -= 1;

                if player.hit_points == 0 {
                    boss_win(result);
                    break
                }
            }
            _ => {}
        };

        if process_effects(player, boss) {
            // Player has won
            player_win(result, player);
            break
        }

        // Cast a spell
        let mut casted = false;

        // Cast missile
        if player.mana >= MISSILE_COST {
            cast_missile(difficulty, result, player.clone(), boss.clone());
            casted = true;
        }

        // Cast drain
        if player.mana >= DRAIN_COST {
            cast_drain(difficulty, result, player.clone(), boss.clone());
            casted = true;
        }

        // Cast shield
        if player.mana >= SHIELD_COST && player.shield_timer == 0 {
            cast_shield(difficulty, result, player.clone(), boss.clone());
            casted = true;
        }

        // Cast poison
        if player.mana >= POISON_COST && player.poison_timer == 0 {
            cast_poison(difficulty, result, player.clone(), boss.clone());
            casted = true;
        }

        // Cast recharge
        if player.mana >= RECHARGE_COST && player.recharge_timer == 0 {
            cast_recharge(difficulty, result, player.clone(), boss.clone());
            casted = true;
        }

        if !casted {
            // If player hasn't got enough mana to cast any spells and no recharge
            // or damage inducing spells are active then boss with inevitably win
            if player.recharge_timer == 0 && player.poison_timer == 0 {
                boss_win(result);
            } else {
                // Boss turn with no cast
                boss_turn(difficulty, result, player, boss);
            }
        }

        break
    };
}

fn cast_missile(difficulty: &Difficulty, result: &mut Result, mut player: PlayerState, mut boss: BossState) {
    player.spend(MISSILE_COST);
    player.casted(Spell::Missile);

    if boss.hit_points <= MISSILE_DAMAGE {
        // Player has won
        player_win(result, &player);
    } else {
        boss.hit_points -= MISSILE_DAMAGE;

        boss_turn(difficulty, result, &mut player, &mut boss);
    }
}

fn cast_drain(difficulty: &Difficulty, result: &mut Result, mut player: PlayerState, mut boss: BossState) {
    player.spend(DRAIN_COST);
    player.casted(Spell::Drain);

    if boss.hit_points <= DRAIN_DAMAGE {
        // Player has won
        player_win(result, &player);
    } else {
        boss.hit_points -= DRAIN_DAMAGE;
        player.hit_points += DRAIN_HEAL;

        boss_turn(difficulty, result, &mut player, &mut boss);
    }
}

fn cast_shield(difficulty: &Difficulty, result: &mut Result, mut player: PlayerState, mut boss: BossState) {
    player.spend(SHIELD_COST);
    player.casted(Spell::Shield);

    player.armor = SHIELD_ARMOR;
    player.shield_timer = SHIELD_TIME;

    boss_turn(difficulty, result, &mut player, &mut boss);
}

fn cast_poison(difficulty: &Difficulty, result: &mut Result, mut player: PlayerState, mut boss: BossState) {
    player.spend(POISON_COST);
    player.casted(Spell::Poison);

    player.poison_timer = POISON_TIME;

    boss_turn(difficulty, result, &mut player, &mut boss);
}

fn cast_recharge(difficulty: &Difficulty, result: &mut Result, mut player: PlayerState, mut boss: BossState) {
    player.spend(RECHARGE_COST);
    player.casted(Spell::Recharge);

    player.recharge_timer = RECHARGE_TIME;

    boss_turn(difficulty, result, &mut player, &mut boss);
}

fn boss_turn(difficulty: &Difficulty, result: &mut Result, player: &mut PlayerState, boss: &mut BossState) {
    loop {
        // Boss's turn
        if process_effects(player, boss) {
            // Player has won
            player_win(result, player);
            break
        };

        if boss_attack(player) {
            // Boss has won
            boss_win(result);
            break
        };

        // Recurse to player turn
        player_turn(difficulty, result, player, boss);
        break
    }
}

fn process_effects(player: &mut PlayerState, boss: &mut BossState) -> bool {
    if player.poison_timer > 0 {
        if boss.hit_points <= POISON_DAMAGE {
            return true
        }
        boss.hit_points -= POISON_DAMAGE;
        player.poison_timer -= 1;
    }

    if player.shield_timer > 0 {
        player.shield_timer -= 1;
        if player.shield_timer == 0 {
            player.armor = 0;
        }
    }

    if player.recharge_timer > 0 {
        player.mana += RECHARGE_MANA;
        player.recharge_timer -= 1;
    }

    false
}

fn boss_attack(player: &mut PlayerState) -> bool {
    let damage = BOSS_DAMAGE - player.armor as u16;

    if player.hit_points <= damage {
        true
    } else {
        player.hit_points -= damage;
        false
    }
}

fn player_win(result: &mut Result, player: &PlayerState) {
    result.player_wins += 1;

    if result.min_spend > player.spend {
        result.min_spend = player.spend;
        result.min_spells = player.spells_cast.clone();
    }
}

fn boss_win(result: &mut Result) {
    result.boss_wins += 1;
}
