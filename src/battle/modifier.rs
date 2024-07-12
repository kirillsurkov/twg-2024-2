#[derive(Debug, Clone)]
pub enum Modifier {
    ChangeAttack(f32),
    AffectHP(f32),
    AffectMana(f32),
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
