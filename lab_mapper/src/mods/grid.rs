use crate::{
	PeriodEnum, PeriodTargetProps,
	mods::scrap::{Identifier, PeriodProps, TimetableProps},
};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ClassTimetableProps {
	pub lecture: String,
	pub lab: Identifier,
	pub block: Identifier,
	pub time: PeriodEnum,
}

pub fn get_class_grid(
	collage_grid: TimetableProps,
	day_targets: &[PeriodTargetProps; 2],
) -> Vec<ClassTimetableProps> {
	let mut out: Vec<ClassTimetableProps> = Vec::new();

	for target in day_targets {
		for block in &collage_grid {
			for lab in &block.labs {
				for period in &lab.periods {
					if is_target(target, period) {
						out.push(ClassTimetableProps {
							lecture: period.lecture.clone(),
							lab: lab.id,
							block: block.id,
							time: period.time.clone(),
						})
					}
				}
			}
		}
	}
	out
}

fn is_target(target_period: &PeriodTargetProps, period: &PeriodProps) -> bool {
	target_period.lecture.eq(&period.lecture) && period.time.eq(&target_period.time)
}
