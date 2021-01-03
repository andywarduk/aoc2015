fn main() {
    part1();
    part2();
}
struct KitProfile {
    name: &'static str,
    cost: u16,
    damage: u16,
    armor: u16
}

const WEAPONS: [KitProfile; 5] = [
    KitProfile { name: "Dagger",     cost:   8, damage: 4, armor: 0 },
    KitProfile { name: "Shortsword", cost:  10, damage: 5, armor: 0 },
    KitProfile { name: "Warhammer",  cost:  25, damage: 6, armor: 0 },
    KitProfile { name: "Longsword",  cost:  40, damage: 7, armor: 0 },
    KitProfile { name: "Greataxe",   cost:  74, damage: 8, armor: 0 },
];

const ARMOR: [KitProfile; 5] = [
    KitProfile { name: "Leather",    cost:  13, damage: 0, armor: 1 },
    KitProfile { name: "Chainmail",  cost:  31, damage: 0, armor: 2 },
    KitProfile { name: "Splintmail", cost:  53, damage: 0, armor: 3 },
    KitProfile { name: "Bandedmail", cost:  75, damage: 0, armor: 4 },
    KitProfile { name: "Platemail",  cost: 102, damage: 0, armor: 5 },
];

const RINGS: [KitProfile; 6] = [
    KitProfile { name: "Damage +1",  cost:  25, damage: 1, armor: 0 },
    KitProfile { name: "Damage +2",  cost:  50, damage: 2, armor: 0 },
    KitProfile { name: "Damage +3",  cost: 100, damage: 3, armor: 0 },
    KitProfile { name: "Defense +1", cost:  20, damage: 0, armor: 1 },
    KitProfile { name: "Defense +2", cost:  40, damage: 0, armor: 2 },
    KitProfile { name: "Defense +3", cost:  80, damage: 0, armor: 3 },
];

#[derive(Clone, Default)]
struct PersonProfile {
    hit_points: u16,
    damage: u16,
    armor: u16
}

// Boss profile from input
const BOSS_PROFILE: PersonProfile = PersonProfile {
    hit_points: 109,
    damage: 8,
    armor: 2
};

#[derive(Default)]
struct Result<'a> {
    cost: u16,
    weapon: Option<&'a KitProfile>,
    armor: Option<&'a KitProfile>,
    ring1: Option<&'a KitProfile>,
    ring2: Option<&'a KitProfile>,
}

impl<'a> Result<'a> {
    fn equipment_list(&self) -> String {
        let mut equipment = Vec::new();

        if let Some(weapon) = self.weapon { equipment.push(format!("Weapon: {}", weapon.name)) };
        if let Some(armor) = self.armor { equipment.push(format!("Armor: {}", armor.name)) };
        if let Some(ring1) = self.ring1 { equipment.push(format!("Ring 1: {}", ring1.name)) };
        if let Some(ring2) = self.ring2 { equipment.push(format!("Ring 2: {}", ring2.name)) };
        
        equipment.join(", ")
    }
}

type Callback<'a> = &'a mut dyn FnMut(&'static KitProfile, Option<&'static KitProfile>,
    Option<&'static KitProfile>, Option<&'static KitProfile>) -> ();

fn part1() {
    let mut result: Result = Default::default();
    result.cost = u16::MAX;

    let mut part1_play = |weapon: &'static KitProfile, armor: Option<&'static KitProfile>,
            ring1: Option<&'static KitProfile>, ring2: Option<&'static KitProfile>| {
        let (mut player_profile, cost) = build_player_profile(weapon, armor, ring1, ring2);

        if cost < result.cost {
            // Play
            if play(&mut player_profile) {
                // Player wins
                result.cost = cost;
                result.weapon = Some(weapon);
                result.armor = armor;
                result.ring1 = ring1;
                result.ring2 = ring2;
            }
        }    
    };

    choose_weapon(&mut part1_play);

    println!("Minimum cost for player win (part 1): {} ({})", result.cost, result.equipment_list());
}

fn part2() {
    let mut result: Result = Default::default();

    let mut part2_play = |weapon: &'static KitProfile, armor: Option<&'static KitProfile>,
            ring1: Option<&'static KitProfile>, ring2: Option<&'static KitProfile>| {
        let (mut player_profile, cost) = build_player_profile(weapon, armor, ring1, ring2);

        if cost > result.cost {    
            // Play
            if !play(&mut player_profile) {
                // Boss wins
                result.cost = cost;
                result.weapon = Some(weapon);
                result.armor = armor;
                result.ring1 = ring1;
                result.ring2 = ring2;
            }
        }    
    };

    choose_weapon(&mut part2_play);

    println!("Maximum cost for boss win (part 2): {} ({})", result.cost, result.equipment_list());
}

#[inline]
fn build_player_profile(weapon: &'static KitProfile, armor: Option<&'static KitProfile>,
        ring1: Option<&'static KitProfile>, ring2: Option<&'static KitProfile>)
        -> (PersonProfile, u16) {
    let mut player_profile: PersonProfile = Default::default();
    let mut cost = 0;

    // Set player hit points
    player_profile.hit_points = 100;

    // Add kit
    let mut add = |e: &KitProfile| {
        player_profile.damage += e.damage;
        player_profile.armor += e.armor;
        cost += e.cost;
    };

    add(weapon);
    if let Some(armor) = armor { add(armor) };
    if let Some(ring1) = ring1 { add(ring1) };
    if let Some(ring2) = ring2 { add(ring2) };

    (player_profile, cost)
}

fn choose_weapon(callback: Callback) {
    // Choice of weapon
    for w in WEAPONS.iter() {
        choose_armor(callback, w);
    }
}

fn choose_armor(callback: Callback, weapon: &'static KitProfile) {
    // No armor
    choose_rings(callback, weapon, None);

    // Choice of armor
    for a in ARMOR.iter() {
        choose_rings(callback, weapon, Some(a));
    }
}

fn choose_rings(callback: Callback, weapon: &'static KitProfile, armor: Option<&'static KitProfile>) {
    // No rings
    equip_and_play(callback, weapon, armor, None, None);

    // One ring
    for r in RINGS.iter() {
        equip_and_play(callback, weapon, armor, Some(r), None);
    }

    // Two rings
    for i in 0..RINGS.len() - 1 {
        for j in i + 1..RINGS.len() {
            equip_and_play(callback, weapon, armor, Some(&RINGS[i]), Some(&RINGS[j]));
        }
    } 
}

#[inline]
fn equip_and_play(callback: Callback, weapon: &'static KitProfile, armor: Option<&'static KitProfile>,
        ring1: Option<&'static KitProfile>, ring2: Option<&'static KitProfile>) {
    callback(weapon, armor, ring1, ring2);
}

fn play(player_profile: &mut PersonProfile) -> bool {
    let mut boss_profile = BOSS_PROFILE.clone();

    let player_damage = if boss_profile.armor >= player_profile.damage { 1 }
    else { player_profile.damage - boss_profile.armor };

    let boss_damage = if player_profile.armor >= boss_profile.damage { 1 }
    else { boss_profile.damage - player_profile.armor };

    loop {
        // Player's turn
        if player_damage >= boss_profile.hit_points {
            break true
        }
        boss_profile.hit_points -= player_damage;

        // Boss's turn
        if boss_damage > player_profile.hit_points {
            break false
        }
        player_profile.hit_points -= boss_damage;
    }
}
