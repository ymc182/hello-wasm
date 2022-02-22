use std::cmp;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
	pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn create_new_character(name: String) -> Character {
	Character::new(name)
}
#[wasm_bindgen]
pub fn create_new_mob(name: String, lvl: i32, hp: i32, ap: i32, dp: i32) -> Character {
	Character::new_mob(name, lvl, hp, ap, dp)
}
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum GearSlot {
	Torso,
	Hand,
	Foot,
}
#[wasm_bindgen]
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Gear {
	name: String,
	ap: Option<i32>,
	dp: Option<i32>,
	hp: Option<i32>,
	slot: GearSlot,
}
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Character {
	name: String,
	lvl: i32,
	hp: i32,
	ap: i32,
	dp: i32,
	exp: Option<i32>,
	slot: HashMap<GearSlot, Gear>,
}

impl Character {
	pub fn new(name: String) -> Character {
		Character {
			name: name,
			lvl: 1,
			hp: 20,
			ap: 5,
			dp: 5,
			exp: Some(0),
			slot: HashMap::new(),
		}
	}
	pub fn new_mob(name: String, lvl: i32, hp: i32, ap: i32, dp: i32) -> Character {
		Character {
			name: name,
			lvl: lvl,
			hp: hp,
			ap: ap,
			dp: dp,
			exp: None,
			slot: HashMap::new(),
		}
	}

	pub fn attack(&self, target: &mut Character) -> bool {
		let pd = cmp::max(self.ap - (target.dp / 2), 1);
		target.hp -= pd;
		println!("{} attacked {} for {:?} HP", self.name, target.name, pd);
		return target.hp <= 0;
	}
	pub fn get_hp(&self) -> i32 {
		return self.hp;
	}
	pub fn equip_item(&mut self, gear: &Gear) {
		let Gear {
			name: _,
			ap,
			hp,
			dp,
			ref slot,
		} = gear;

		self.ap += ap.unwrap_or_else(|| 0);
		self.dp += dp.unwrap_or_else(|| 0);
		self.hp += hp.unwrap_or_else(|| 0);
		self.slot.insert(*slot, gear.clone());
	}
	pub fn remove_item(&mut self, slot: GearSlot) {
		let gear = self.slot.remove(&slot);
		if !gear.is_none() {
			let gear = gear.unwrap();
			let Gear {
				name: _,
				ap,
				hp,
				dp,
				slot: _,
			} = gear;
			self.ap -= ap.unwrap_or_else(|| 0);
			self.dp -= dp.unwrap_or_else(|| 0);
			self.hp -= hp.unwrap_or_else(|| 0);
		}
	}
}

impl Gear {
	pub fn new(
		name: String,
		ap: Option<i32>,
		dp: Option<i32>,
		hp: Option<i32>,
		slot: GearSlot,
	) -> Gear {
		Gear {
			name: name,
			ap: ap,
			dp: dp,
			hp: hp,
			slot: slot,
		}
	}
}

#[cfg(test)]

mod tests {
	use super::*;
	#[test]
	fn can_equip_item() {
		let ref mut character = Character::new("Eric".to_string());
		let ref mut mob = Character::new_mob("Goblin".to_string(), 3, 9, 3, 3);
		let ref gear = Gear::new("Axe".to_string(), Some(3), Some(5), None, GearSlot::Hand);
		character.equip_item(gear);
		while !character.attack(mob) && !mob.attack(character) {
			println!("{} has {} hp remain", &character.name, &character.hp);
			println!("{} has {} hp remain", &mob.name, &mob.hp);
		}
		println!(
			"Equiped  {}",
			&character.slot.get(&gear.slot.clone()).unwrap().name
		);
	}
}
