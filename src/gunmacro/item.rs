// gunmacro/item

use std::fmt::{self, Debug, Display};
use super::Class;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::EnumIter)]
pub enum Item {
	// gun
	#[default] Pistol = 1,
	Shotgun,
	Rifle,
	Revolver,
	Flintlock,
	Ak47,
	Sword,
	Uzi,
	Forcefield,
	PlasmaPistol,
	PlasmaShotgun,
	Sniper,
	// explosive
	C4 = 21,
	SmokeGrenade = 23,
	Grenade = 25,
	RocketLauncher = 28,
	// misc
	Flashlight = 31,
	Binoculars = 32,
}

impl Item {
	pub fn class(self) -> Class {
		use Item::*;
		match self {
			// guns
			Pistol | Shotgun | Rifle | Revolver | Flintlock
			| Ak47 | Sword | Uzi | Forcefield | PlasmaPistol
			| PlasmaShotgun | Sniper => Class::Gun,

			// explosive
			C4 | SmokeGrenade | Grenade | RocketLauncher => Class::Explosive,

			// misc
			Flashlight | Binoculars => Class::Misc,
		}
	}

	pub fn order(self) -> i32 {
		match self.class() {
			Class::Gun => self as i32,
			Class::Explosive => self as i32 - 20,
			Class::Misc => self as i32 - 30
		}
	}
}

impl Display for Item {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use Item::*;
		f.write_str(match self {
			Pistol => "Pistol",
			Shotgun => "Shotgun",
			Rifle => "Rifle",
			Revolver => "Revolver",
			Flintlock => "Flintlock",
			Ak47 => "AK-47",
			Sword => "Sword",
			Uzi => "Uzi",
			Forcefield => "Forcefield",
			PlasmaPistol => "Plasma Pistol",
			PlasmaShotgun => "Plasma Shotgun",
			Sniper => "Sniper",

			C4 => "C4",
			SmokeGrenade => "Smoke Grenades",
			Grenade => "Grenades",
			RocketLauncher => "Rocket Launcher",

			Flashlight => "Flashlight",
			Binoculars => "Binoculars",
		})
	}
}