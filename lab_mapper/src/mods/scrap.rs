use crate::{PeriodEnum, mods::log::Log};
use std::collections::HashSet;

pub type TimetableProps = HashSet<BlockProps>;
pub type Identifier = i32;

const OPEN_TAG_DELIMITER: char = '<';
const CLOSE_TAG_DELIMITER: char = '>';
const BLOCK_IDENTIFIER: &str = "BLOCO";
const LAB_IDENTIFIER: &str = "LAB";
const PERIOD_DELIMITER: &str = "AULA";
const IGNORE_TAGS: &[&str] = &[
	"</div",
	"</td",
	"</tr",
	"</table",
	"</span",
	"<br",
	"<tr",
	"<td",
	"<th",
	"<table",
	"<div",
	r#"<td class="periodo""#,
	r#"| <span class="spacer""#,
	r#"1º Horário</div"#,
	r#"<table class="bloco""#,
	r#"2º Horário <span class="spacer""#,
	r#"<div class="reserva ""#,
	r#"<td valign="top" class="lab""#,
	r#"<table class="tableReserva""#,
];

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct PeriodProps {
	pub time: PeriodEnum,
	pub lecture: String,
	pub class: Option<String>,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct LabProps {
	pub id: Identifier,
	pub periods: Vec<PeriodProps>,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct BlockProps {
	pub id: Identifier,
	pub labs: Vec<LabProps>,
}

pub fn extract_timetable(input: String) -> std::io::Result<TimetableProps> {
	let mut out: TimetableProps = HashSet::new();
	let mut registering_block: Option<BlockProps> = None;
	let mut registering_lab: Option<LabProps> = None;
	let mut registering_period: Option<PeriodProps> = None;
	let mut period_time: PeriodEnum = PeriodEnum::First;
	let mut is_registering_lab: bool = true;
	let splited_table = get_last_line(input.as_str())?;

	for i in splited_table {
		if IGNORE_TAGS.contains(&i.trim()) {
			continue;
		}

		if i.contains(BLOCK_IDENTIFIER) {
			is_registering_lab = false;
			if let Some(b) = registering_block {
				println!("{}", format!("{:?}", b.clone()).contains("CARLOS LUZ"));
				out.insert(b);
			}

			registering_block = Some(BlockProps {
				id: get_id(i, BLOCK_IDENTIFIER)?,
				labs: Vec::new(),
			});
		}

		if i.contains(LAB_IDENTIFIER) {
			is_registering_lab = true;

			if let Some(l) = registering_lab.clone()
				&& let Some(mut rb) = registering_block
			{
				rb.labs.push(l);
				registering_block = Some(rb);
			}
			registering_lab = Some(LabProps {
				id: get_id(i, LAB_IDENTIFIER)?,
				periods: Vec::new(),
			});
		}
		if i.contains(PERIOD_DELIMITER) {
			period_time = if period_time.eq(&PeriodEnum::First) {
				PeriodEnum::Second
			} else {
				PeriodEnum::First
			};
		}

		if is_registering_lab
			&& !i.contains(LAB_IDENTIFIER)
			&& let Some(mut lab) = registering_lab.clone()
		{
			if remove_tag(i) == PERIOD_DELIMITER {
				registering_period = None;
			} else if let Some(mut rp) = registering_period.clone() {
				rp.class = Some(remove_tag(i));
				rp.time = period_time.clone();
				lab.periods.push(rp);
				registering_lab = Some(lab);
			} else {
				registering_period = Some(PeriodProps {
					lecture: remove_tag(i),
					time: period_time.clone(),
					class: None,
				});
			};
		}
	}

	// apanhei umas horas pra isso aqui. la em cima eu registrava o bloco somente quando o divisor
	// fosse encontrado, quando chegava no ultimo n tinha divisor na proxima linha, entao o ultimo
	// bloco nunca era registrado, essa linha vem pra resolver isso:
	if let Some(rb) = registering_block {
		out.insert(rb);
	}

	Ok(out)
}

fn remove_tag(input: &str) -> String {
	let mut out = input.to_string();
	for i in IGNORE_TAGS {
		out = out.replace(i, "");
	}
	out
}

fn get_id(i: &str, identifier: &str) -> std::io::Result<Identifier> {
	((i.split(identifier)
		.last()
		.ok_or_else(|| Log::err("Could not get block id"))?)
	.split(OPEN_TAG_DELIMITER)
	.collect::<Vec<&str>>())[0]
		.trim()
		.parse::<Identifier>()
		.map_err(|err| Log::err(err.to_string()))
}

fn get_last_line(input: &str) -> std::io::Result<Vec<&str>> {
	let a = (input
		.lines()
		.last()
		.ok_or_else(|| Log::err("Invalid html input data"))?)
	.split(CLOSE_TAG_DELIMITER)
	.collect::<Vec<&str>>();
	Ok(a)
}
