/// Categoria do nó na árvore de habilidades.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum SkillCluster {
    Speed,
    Offense,
    Defense,
    Energy,
    Utility,
    Bonus,
}

impl SkillCluster {
    pub const ALL: [SkillCluster; 6] = [
        SkillCluster::Speed,
        SkillCluster::Offense,
        SkillCluster::Defense,
        SkillCluster::Energy,
        SkillCluster::Utility,
        SkillCluster::Bonus,
    ];

    pub fn name(self) -> &'static str {
        match self {
            SkillCluster::Speed   => "SPEED",
            SkillCluster::Offense => "OFFENSE",
            SkillCluster::Defense => "DEFENSE",
            SkillCluster::Energy  => "ENERGY",
            SkillCluster::Utility => "UTILITY",
            SkillCluster::Bonus   => "BONUS",
        }
    }
}

/// Efeito aplicado ao desbloquear o nó.
#[derive(Clone, Copy)]
pub enum SkillEffect {
    SpeedMul(f32),
    DamageMul(f32),
    FireRateMul(f32),
    BulletSpeedMul(f32),
    MaxHpFlat(f32),
    MaxShieldFlat(f32),
    MaxEnergyFlat(f32),
    PickupRadius(f32),
    HpRegen(f32),
    ShieldRegenMul(f32),
    EnergyRegenMul(f32),
}

/// Definição de um nó da skill tree.
pub struct SkillNodeDef {
    pub id: u32,
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: u32,
    pub cluster: SkillCluster,
    pub effect: SkillEffect,
    /// Pré-requisito: ID do nó que precisa estar desbloqueado antes.
    pub prereq: Option<u32>,
}

fn n(id: u32, name: &'static str, desc: &'static str, cost: u32,
     cluster: SkillCluster, effect: SkillEffect, prereq: Option<u32>) -> SkillNodeDef {
    SkillNodeDef { id, name, desc, cost, cluster, effect, prereq }
}

/// Retorna todos os 101 nós da skill tree.
pub fn all_nodes() -> Vec<SkillNodeDef> {
    use SkillCluster::*;
    use SkillEffect::*;
    // Custo tiered: 1→8, 2→15, 3→25, 4→40, 5→60
    // Total para maxar tudo: ~2400 SP. Max disponivel em 25 cenarios: ~1800 SP.
    // Impossivel maxar tudo — o jogador precisa escolher com sabedoria.
    vec![
        // ── Speed (0-10, 11 nodes) ──────────────────────────────────────────
        n(0,  "Light Thrusters",   "+8% speed",    8, Speed, SpeedMul(1.08), None),
        n(1,  "Efficient Engines", "+12% speed",   8, Speed, SpeedMul(1.12), Some(0)),
        n(2,  "Agile Frame",       "+10% speed",  15, Speed, SpeedMul(1.10), None),
        n(3,  "Quick Maneuvering", "+12% speed",  15, Speed, SpeedMul(1.12), Some(2)),
        n(4,  "Turbo Boost",       "+15% speed",  15, Speed, SpeedMul(1.15), Some(1)),
        n(5,  "Afterburner",       "+20% speed",  25, Speed, SpeedMul(1.20), Some(4)),
        n(6,  "Lightweight Hull",  "+12% speed",  15, Speed, SpeedMul(1.12), Some(3)),
        n(7,  "Speed Demon",       "+18% speed",  25, Speed, SpeedMul(1.18), Some(5)),
        n(8,  "Nitro Jets",        "+25% speed",  40, Speed, SpeedMul(1.25), Some(7)),
        n(9,  "Hyperdrive",        "+35% speed",  40, Speed, SpeedMul(1.35), Some(8)),
        n(10, "Light Speed",       "+50% speed",  60, Speed, SpeedMul(1.50), Some(9)),
        // ── Offense (11-24, 14 nodes) ───────────────────────────────────────
        n(11, "Sharp Rounds",      "+8% damage",        8, Offense, DamageMul(1.08), None),
        n(12, "Armor Piercing",    "+12% damage",       8, Offense, DamageMul(1.12), Some(11)),
        n(13, "Explosive Tips",    "+16% damage",      15, Offense, DamageMul(1.16), Some(12)),
        n(14, "Hollow Point",      "+20% damage",      15, Offense, DamageMul(1.20), Some(13)),
        n(15, "Annihilator",       "+25% damage",      25, Offense, DamageMul(1.25), Some(14)),
        n(16, "Destroyer",         "+35% damage",      40, Offense, DamageMul(1.35), Some(15)),
        n(17, "Quick Trigger",     "+8% fire rate",     8, Offense, FireRateMul(1.08), None),
        n(18, "Rapid Fire",        "+12% fire rate",   15, Offense, FireRateMul(1.12), Some(17)),
        n(19, "Hair Trigger",      "+15% fire rate",   15, Offense, FireRateMul(1.15), Some(18)),
        n(20, "Gatling Mode",      "+20% fire rate",   25, Offense, FireRateMul(1.20), Some(19)),
        n(21, "Bullet Velocity",   "+10% bullet spd",   8, Offense, BulletSpeedMul(1.10), None),
        n(22, "Kinetic Boost",     "+15% bullet spd",  15, Offense, BulletSpeedMul(1.15), Some(21)),
        n(23, "Hypervelocity",     "+20% bullet spd",  25, Offense, BulletSpeedMul(1.20), Some(22)),
        n(24, "Killing Machine",   "+30% damage",      60, Offense, DamageMul(1.30), Some(16)),
        // ── Defense (25-38, 14 nodes) ───────────────────────────────────────
        n(25, "Reinforced Hull",   "+20 HP",             8, Defense, MaxHpFlat(20.0),    None),
        n(26, "Armor Plating",     "+30 HP",            15, Defense, MaxHpFlat(30.0),    Some(25)),
        n(27, "Battle Hardened",   "+50 HP",            15, Defense, MaxHpFlat(50.0),    Some(26)),
        n(28, "Iron Skin",         "+80 HP",            25, Defense, MaxHpFlat(80.0),    Some(27)),
        n(29, "Juggernaut",        "+120 HP",           40, Defense, MaxHpFlat(120.0),   Some(28)),
        n(30, "Shield Bank",       "+15 shield",         8, Defense, MaxShieldFlat(15.0), None),
        n(31, "Heavy Shields",     "+25 shield",        15, Defense, MaxShieldFlat(25.0), Some(30)),
        n(32, "Shield Wall",       "+40 shield",        25, Defense, MaxShieldFlat(40.0), Some(31)),
        n(33, "Fortress",          "+60 shield",        40, Defense, MaxShieldFlat(60.0), Some(32)),
        n(34, "Shield Regen",      "+50% shld regen",   15, Defense, ShieldRegenMul(1.50), None),
        n(35, "Quick Recovery",    "+75% shld regen",   25, Defense, ShieldRegenMul(1.75), Some(34)),
        n(36, "Toughness",         "+40 HP",            15, Defense, MaxHpFlat(40.0),    None),
        n(37, "Resilience",        "+60 HP",            25, Defense, MaxHpFlat(60.0),    Some(36)),
        n(38, "Invulnerable",      "+80 shield",        40, Defense, MaxShieldFlat(80.0), Some(33)),
        // ── Energy (39-50, 12 nodes) ────────────────────────────────────────
        n(39, "Power Cell",        "+20 energy",         8, Energy, MaxEnergyFlat(20.0),  None),
        n(40, "Backup Battery",    "+30 energy",        15, Energy, MaxEnergyFlat(30.0),  Some(39)),
        n(41, "Energy Core",       "+50 energy",        15, Energy, MaxEnergyFlat(50.0),  Some(40)),
        n(42, "Reactor Upgrade",   "+80 energy",        25, Energy, MaxEnergyFlat(80.0),  Some(41)),
        n(43, "Quick Charge",      "+25% nrg regen",     8, Energy, EnergyRegenMul(1.25), None),
        n(44, "Overloaded Reactor","+50% nrg regen",    15, Energy, EnergyRegenMul(1.50), Some(43)),
        n(45, "Perpetual Motion",  "+75% nrg regen",    25, Energy, EnergyRegenMul(1.75), Some(44)),
        n(46, "Supercharger",      "+100% nrg regen",   40, Energy, EnergyRegenMul(2.00), Some(45)),
        n(47, "Capacitor Array",   "+30 energy",        15, Energy, MaxEnergyFlat(30.0),  None),
        n(48, "High Voltage",      "+40 energy",        25, Energy, MaxEnergyFlat(40.0),  Some(47)),
        n(49, "Megawatt Core",     "+60 energy",        40, Energy, MaxEnergyFlat(60.0),  Some(48)),
        n(50, "Infinite Power",    "+100 energy",       60, Energy, MaxEnergyFlat(100.0), Some(49)),
        // ── Utility (51-62, 12 nodes) ───────────────────────────────────────
        n(51, "Scavenger",         "+20 pickup radius",  8, Utility, PickupRadius(20.0),    None),
        n(52, "Magnetar",          "+30 pickup radius", 15, Utility, PickupRadius(30.0),    Some(51)),
        n(53, "Collector",         "+50 pickup radius", 25, Utility, PickupRadius(50.0),    Some(52)),
        n(54, "HP Regen I",        "+2 HP/sec",         15, Utility, HpRegen(2.0),          None),
        n(55, "HP Regen II",       "+3 HP/sec",         25, Utility, HpRegen(3.0),          Some(54)),
        n(56, "Vitality",          "+5 HP/sec",         40, Utility, HpRegen(5.0),          Some(55)),
        n(57, "Wide Sensors",      "+25 pickup radius", 15, Utility, PickupRadius(25.0),    None),
        n(58, "Advanced Sensors",  "+35 pickup radius", 25, Utility, PickupRadius(35.0),    Some(57)),
        n(59, "Emergency Repair",  "+1 HP/sec",          8, Utility, HpRegen(1.0),          None),
        n(60, "Regeneration",      "+2 HP/sec",         15, Utility, HpRegen(2.0),          Some(59)),
        n(61, "Shield Boost",      "+50% shld regen",   15, Utility, ShieldRegenMul(1.50),  None),
        n(62, "Rapid Shields",     "+75% shld regen",   25, Utility, ShieldRegenMul(1.75),  Some(61)),
        // ── Bonus (63-100, 38 nodes) ─────────────────────────────────────────
        n(63, "Combat Instincts",  "+10% damage",       15, Bonus, DamageMul(1.10),     None),
        n(64, "Pilot Training",    "+10% speed",        15, Bonus, SpeedMul(1.10),      None),
        n(65, "Endurance",         "+30 HP",            15, Bonus, MaxHpFlat(30.0),     None),
        n(66, "Power Reserves",    "+25 energy",        15, Bonus, MaxEnergyFlat(25.0), None),
        n(67, "Battle Focus",      "+10% fire rate",    15, Bonus, FireRateMul(1.10),   None),
        n(68, "Reaction Time",     "+8% bullet spd",    15, Bonus, BulletSpeedMul(1.08), None),
        n(69, "Shield Proficiency","+20 shield",        15, Bonus, MaxShieldFlat(20.0), None),
        n(70, "Survivor",          "+40 HP",            25, Bonus, MaxHpFlat(40.0),     Some(65)),
        n(71, "Combat Veteran",    "+15% damage",       25, Bonus, DamageMul(1.15),     Some(63)),
        n(72, "Ace Pilot",         "+15% speed",        25, Bonus, SpeedMul(1.15),      Some(64)),
        n(73, "Power Management",  "+35 energy",        25, Bonus, MaxEnergyFlat(35.0), Some(66)),
        n(74, "Trigger Discipline","+15% fire rate",    25, Bonus, FireRateMul(1.15),   Some(67)),
        n(75, "Ballistics Expert", "+12% bullet spd",   25, Bonus, BulletSpeedMul(1.12), Some(68)),
        n(76, "Shield Master",     "+30 shield",        25, Bonus, MaxShieldFlat(30.0), Some(69)),
        n(77, "War Machine",       "+20% damage",       40, Bonus, DamageMul(1.20),     Some(71)),
        n(78, "Speed Merchant",    "+20% speed",        40, Bonus, SpeedMul(1.20),      Some(72)),
        n(79, "Energy Master",     "+50 energy",        40, Bonus, MaxEnergyFlat(50.0), Some(73)),
        n(80, "Rapid Deployment",  "+20% fire rate",    40, Bonus, FireRateMul(1.20),   Some(74)),
        n(81, "Kinetic Specialist","+18% bullet spd",   40, Bonus, BulletSpeedMul(1.18), Some(75)),
        n(82, "Shield Architect",  "+50 shield",        40, Bonus, MaxShieldFlat(50.0), Some(76)),
        n(83, "Overpower",         "+12% damage",       15, Bonus, DamageMul(1.12),     None),
        n(84, "Phase Drive",       "+10% speed",        15, Bonus, SpeedMul(1.10),      None),
        n(85, "Bulwark",           "+25 HP",            15, Bonus, MaxHpFlat(25.0),     None),
        n(86, "Cell Upgrade",      "+20 energy",        15, Bonus, MaxEnergyFlat(20.0), None),
        n(87, "Steady Aim",        "+10% fire rate",    15, Bonus, FireRateMul(1.10),   None),
        n(88, "Precision",         "+10% bullet spd",   15, Bonus, BulletSpeedMul(1.10), None),
        n(89, "Energy Shield",     "+15 shield",        15, Bonus, MaxShieldFlat(15.0), None),
        n(90, "Obliterator",       "+15% damage",       25, Bonus, DamageMul(1.15),     Some(83)),
        n(91, "Quantum Sprint",    "+12% speed",        25, Bonus, SpeedMul(1.12),      Some(84)),
        n(92, "Titanium Frame",    "+35 HP",            25, Bonus, MaxHpFlat(35.0),     Some(85)),
        n(93, "Power Amplifier",   "+30 energy",        25, Bonus, MaxEnergyFlat(30.0), Some(86)),
        n(94, "Aim Assist",        "+12% fire rate",    25, Bonus, FireRateMul(1.12),   Some(87)),
        n(95, "Rail Driver",       "+15% bullet spd",   25, Bonus, BulletSpeedMul(1.15), Some(88)),
        n(96, "Reactive Armor",    "+25 shield",        25, Bonus, MaxShieldFlat(25.0), Some(89)),
        n(97, "Annihilation",      "+18% damage",       40, Bonus, DamageMul(1.18),     Some(90)),
        n(98, "Hypersprint",       "+15% speed",        40, Bonus, SpeedMul(1.15),      Some(91)),
        n(99, "Nano Armor",        "+50 HP",            40, Bonus, MaxHpFlat(50.0),     Some(92)),
        n(100,"Cascade Fire",      "+18% fire rate",    40, Bonus, FireRateMul(1.18),   Some(94)),
    ]
}
