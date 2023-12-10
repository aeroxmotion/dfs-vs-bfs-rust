use crate::base::{get_spell, Boss, Player, Turn, SPELLS};

pub fn _simulate(player: Player, boss: Boss, turn: Turn, spent_min: i32, hard: bool) -> i32 {
	let mut next_boss = boss.clone();
	let mut next_player = player.clone();

	if hard && matches!(turn, Turn::Player) {
		next_player.life -= 1;

		if next_player.dead() {
			return spent_min;
		}
	}

	if next_player.consume("poison") && next_player.attack(&mut next_boss, get_spell("poison")) {
		return spent_min.min(next_player.spent);
	}

	if next_player.consume("recharge") {
		next_player.mana += get_spell("recharge").mana;
	}

	let armor = if next_player.consume("shield") {
		get_spell("shield").armor
	} else {
		0
	};

	if matches!(turn, Turn::Boss) {
		return if next_boss.attack(&mut next_player, armor) {
			i32::MAX
		} else {
			_simulate(next_player, next_boss, Turn::Player, spent_min, hard)
		};
	}

	let mut min = spent_min;

	for (_, spell) in SPELLS.iter() {
		if spell.cost > next_player.mana
			|| next_player.spent + spell.cost > min
			|| next_player.active(spell.name)
		{
			continue;
		}

		let mut deep_boss = next_boss.clone();
		let mut deep_player = next_player.clone();

		// Mana
		deep_player.mana -= spell.cost;
		deep_player.spent += spell.cost;

		// Is an effect?
		if spell.turns > 0 {
			deep_player.start(spell);
		} else if deep_player.attack(&mut deep_boss, spell) {
			min = min.min(deep_player.spent);
			continue;
		}

		min = min.min(_simulate(deep_player, deep_boss, Turn::Boss, min, hard));
	}

	return min;
}

pub fn simulate(hard: bool) -> i32 {
	_simulate(Player::new(), Boss::new(), Turn::Player, i32::MAX, hard)
}

#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

	#[test]
	fn it_should_work_on_easy_mode() {
		assert_eq!(simulate(false), 900);
	}

	#[bench]
	fn bench_on_easy_mode(b: &mut Bencher) {
		b.iter(|| simulate(false));
	}

	#[test]
	fn it_should_work_on_hard_mode() {
		assert_eq!(simulate(true), 1_216);
	}

	#[bench]
	fn bench_on_hard_mode(b: &mut Bencher) {
		b.iter(|| simulate(true));
	}
}
