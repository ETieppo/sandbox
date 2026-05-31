use chrono::{Datelike, NaiveDate};

pub async fn get_html(now: NaiveDate) -> Result<String, reqwest::Error> {
	let url = format!(
		"https://app.unicesumar.edu.br/presencial/forms/informatica/horario.php\\?dados\\={}-{}-{}\\|N",
		now.year(),
		now.month(),
		now.day(),
	);

	let res = reqwest::get(url).await?.text().await?;
	Ok(res)
}
