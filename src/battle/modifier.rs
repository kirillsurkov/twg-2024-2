#[derive(Debug, Clone)]
pub enum Modifier {
    AffectAttack(f32),
    AffectAttackSpeed(f32),
    AffectHP(f32),
    AffectMana(f32),
    // markers
    NormalAttack,
    SpawnSwiborg(u32),
    ShootSwiborg(u32),
    ShootDuck,
    SpawnFireCube(u32),
    ShootFireCube(u32),
    ShootHealBeam,
    ShootDamageBeam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    Myself,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueKind {
    Units,
    Percents,
}

#[derive(Debug, Clone)]
pub struct ModifierDesc {
    pub modifier: Modifier,
    pub target: Target,
    pub value_kind: ValueKind,
}

impl ModifierDesc {
    pub fn key(&self) -> impl Ord {
        (self.target, self.value_kind)
    }
}
