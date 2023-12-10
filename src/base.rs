use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct Spell<'a> {
	pub name: &'a str,
	pub cost: i32,
	pub mana: i32,
	pub heal: i32,
	pub armor: i32,
	pub turns: i32,
	pub damage: i32,
}

pub static SPELLS: Lazy<HashMap<&str, Spell>> = Lazy::new(|| {
	HashMap::from([
		(
			"missile",
			Spell {
				name: "missile",
				cost: 53,
				damage: 4,
				..Default::default()
			},
		),
		(
			"drain",
			Spell {
				name: "drain",
				cost: 73,
				damage: 2,
				heal: 2,
				..Default::default()
			},
		),
		(
			"shield",
			Spell {
				name: "shield",
				cost: 113,
				turns: 6,
				armor: 7,
				..Default::default()
			},
		),
		(
			"poison",
			Spell {
				name: "poison",
				cost: 173,
				turns: 6,
				damage: 3,
				..Default::default()
			},
		),
		(
			"recharge",
			Spell {
				name: "recharge",
				cost: 229,
				turns: 5,
				mana: 101,
				..Default::default()
			},
		),
	])
});

pub fn get_spell(name: &str) -> &Spell {
	SPELLS.get(name).unwrap()
}

pub enum Turn {
	Boss,
	Player,
}

#[derive(Clone, Debug)]
pub struct Boss {
	pub life: i32,
	pub damage: i32,
}

impl Boss {
	pub fn new() -> Boss {
		Boss {
			life: 51,
			damage: 9,
		}
	}

	///
	pub fn dead(&self) -> bool {
		self.life <= 0
	}

	///
	pub fn attack(&self, player: &mut Player, armor: i32) -> bool {
		player.life -= 1.max(self.damage - armor);
		player.dead()
	}
}

#[derive(Clone, Debug)]
pub struct Player<'a> {
	pub life: i32,
	pub mana: i32,
	pub spent: i32,
	pub effects: HashMap<&'a str, i32>,
}

impl<'a> Player<'a> {
	/// Returns new `Player` instance (with defaults).
	pub fn new() -> Player<'a> {
		let mut effects = HashMap::new();

		for (effect, spell) in SPELLS.iter() {
			if spell.turns > 0 {
				effects.insert(*effect, 0);
			}
		}

		Player {
			life: 50,
			mana: 500,
			spent: 0,
			effects,
		}
	}

	/// Returns true whether the `Player` is dead.
	pub fn dead(&self) -> bool {
		self.life <= 0
	}

	/// Activate given `Spell` (effect).
	pub fn start(&mut self, spell: &'a Spell) {
		self.effects.insert(spell.name, spell.turns);
	}

	/// Attack given `Boss` instance with the given `spell`
	pub fn attack(&mut self, boss: &mut Boss, spell: &Spell) -> bool {
		boss.life -= spell.damage;
		self.life += spell.heal;

		boss.dead()
	}

	/// Returns `true` whether the given `effect` is active.
	pub fn active(&self, effect: &'a str) -> bool {
		match self.effects.get(effect) {
			None => false,
			Some(v) => *v > 0,
		}
	}

	/// Consume given `effect` turns.
	/// Returns `true` if given `effect` has been consumed.
	pub fn consume(&mut self, effect: &'a str) -> bool {
		match self.effects.get_mut(effect) {
			None => false,
			Some(v) => {
				if *v <= 0 {
					false
				} else {
					*v -= 1;
					true
				}
			}
		}
	}
}
