mod mods;

use crate::mods::Log;
use chrono::Datelike;
use chrono::Weekday;
use mods::grid;
use mods::http;
use mods::scrap;

const LECTURE_TARGET: &[&str] = &["JOAO BIAZOTTO", "ELIEL NASCIMENTO", "DOUGLAS LOPES", "CARLOS LUZ"];
const ON_WEEK_DAY_TARGETS: &[[PeriodTargetProps; 2]; 5] = &[
	[
		PeriodTargetProps {
			lecture: LECTURE_TARGET[3],
			time: PeriodEnum::First,
		},
		PeriodTargetProps {
			lecture: LECTURE_TARGET[0],
			time: PeriodEnum::Second,
		},
	],
	[
		PeriodTargetProps {
			lecture: LECTURE_TARGET[0],
			time: PeriodEnum::First,
		},
		PeriodTargetProps {
			lecture: LECTURE_TARGET[1],
			time: PeriodEnum::Second,
		},
	],
	[
		PeriodTargetProps {
			lecture: LECTURE_TARGET[2],
			time: PeriodEnum::First,
		},
		PeriodTargetProps {
			lecture: LECTURE_TARGET[3],
			time: PeriodEnum::Second,
		},
	],
	[
		PeriodTargetProps {
			lecture: LECTURE_TARGET[1],
			time: PeriodEnum::First,
		},
		PeriodTargetProps {
			lecture: LECTURE_TARGET[2],
			time: PeriodEnum::Second,
		},
	],
	[
		PeriodTargetProps {
			lecture: "FREE",
			time: PeriodEnum::Free,
		},
		PeriodTargetProps {
			lecture: "FREE",
			time: PeriodEnum::Free,
		},
	],
];

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum PeriodEnum {
	First,
	Second,
	Free,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct PeriodTargetProps {
	lecture: &'static str,
	time: PeriodEnum,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let now = chrono::Local::now().date_naive();

	if now.weekday().eq(&Weekday::Sat) || now.weekday().eq(&Weekday::Sun) {
		println!("Weekend day! stopping...");
		return Ok(());
	};

	let input: String = http::get_html(now)
		.await
		.map_err(|a| Log::err(format!("Error getting server data {}", a)))?
		.replace("\\", "");
	let day_targets: Option<&[PeriodTargetProps; 2]> = match now.weekday() {
		Weekday::Mon => Some(&ON_WEEK_DAY_TARGETS[0]),
		Weekday::Tue => Some(&ON_WEEK_DAY_TARGETS[1]),
		Weekday::Wed => Some(&ON_WEEK_DAY_TARGETS[2]),
		Weekday::Thu => Some(&ON_WEEK_DAY_TARGETS[3]),
		Weekday::Fri => Some(&ON_WEEK_DAY_TARGETS[4]),
		_ => None,
	};
	let scrap_timetable_res = scrap::extract_timetable(input)?;

	match day_targets {
		Some(t) => {
			let grid_class_res = grid::get_class_grid(scrap_timetable_res, t);
			println!("{grid_class_res:?}");
			Ok(())
		}
		None => Err(Log::err("No class periods was provided")),
	}
}
