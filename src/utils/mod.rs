pub mod helpers {
    use crate::structs::Contributor;

    pub fn handle_split(contributors: Vec<Contributor>, bill_to_split: &f64) -> Vec<Contributor> {
        let mut results = vec![];
        for mut c in contributors {
            let amount_to_pay = ((c.percentage as f64 / 100.0) * bill_to_split).round();
            c.amount_contributed = amount_to_pay;
            results.push(c);
        }
        results
    }
}
