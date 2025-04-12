use std::fmt::Display;

pub(crate) trait DisplayCommandAsRow {
    fn args(&self) -> Vec<String>;
    fn description(&self) -> String;
    fn name(&self) -> String;
}

#[derive(PartialEq, Eq)]
pub(crate) struct Row {
    args: Vec<String>,
    description: String,
    name: String,
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Row {
    pub(crate) fn new(cmd: impl DisplayCommandAsRow) -> Self {
        Self {
            args: cmd.args(),
            description: cmd.description(),
            name: cmd.name(),
        }
    }

    fn size_by_param(&self) -> Vec<(String, usize)> {
        vec![
            ("name".to_string(), self.name.len()),
            ("args".to_string(), self.args.join(" ").len()),
            ("description".to_string(), self.description.len()),
        ]
    }
}

struct Section {
    title: String,
    elements: Vec<Row>,
    max_size_by_param: Vec<(String, usize)>,
}

impl Section {
    fn new(title: impl ToString, elements: Vec<Row>) -> Self {
        let mut max_size_by_param = elements[0].size_by_param();
        let _ = &elements[1..].iter().for_each(|e| {
            max_size_by_param = max_of_params(&max_size_by_param, &e.size_by_param())
        });

        Self {
            title: title.to_string(),
            elements,
            max_size_by_param,
        }
    }

    fn pad(&self, s: &str, param_size_idx: usize) -> String {
        let mut res = s.to_string();
        let pad_len = self.max_size_by_param[param_size_idx].1 - s.len();
        res += &" ".repeat(pad_len);
        res
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            writeln!(f, "{}", self.title)?;
            for element in &self.elements {
                writeln!(
                    f,
                    " {} {} {}",
                    self.pad(element.name.as_str(), 0),
                    self.pad(element.args.join(" ").as_str(), 1),
                    self.pad(element.description.as_str(), 2)
                )?;
            }
        }
        Ok(())
    }
}

pub(crate) struct UsageTable {
    title: String,
    sections: Vec<Section>,
}

impl UsageTable {
    fn new(title: String, sections: Vec<Section>) -> Self {
        Self { title, sections }
    }
}

pub(crate) struct UsageTableBuilder {
    title: String,
    sections: Option<Vec<Section>>,
}

impl UsageTableBuilder {
    pub(crate) fn new(title: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            sections: None,
        }
    }

    pub(crate) fn add_section(mut self, title: impl ToString, elements: Vec<Row>) -> Self {
        let section = Section::new(title, elements);
        if let Some(sections) = &mut self.sections {
            sections.push(section);
        } else {
            self.sections = Some(vec![section]);
        }
        self
    }

    pub(crate) fn build(self) -> UsageTable {
        UsageTable::new(self.title, self.sections.unwrap_or_default())
    }
}

impl Display for UsageTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            writeln!(f, "{}", self.title)?;
            writeln!(f, "  tap <command> <args> [options]\n")?;
            for section in &self.sections {
                section.fmt(f)?
            }
        }
        Ok(())
    }
}

// Assumes a and b have same elements in same order. If one vec is longer than the other, then it compares same elements for both and then appends the rest
fn max_of_params(a: &Vec<(String, usize)>, b: &Vec<(String, usize)>) -> Vec<(String, usize)> {
    let (bigger, smaller) = if a.len() > b.len() { (a, b) } else { (b, a) };
    let mut res: Vec<(String, usize)> = Vec::with_capacity(bigger.len());
    for idx in 0..smaller.len() {
        let big_cur = &bigger[idx];
        let small_cur = &smaller[idx];
        res.push((small_cur.0.to_string(), small_cur.1.max(big_cur.1)));
    }
    res.append(&mut bigger[smaller.len()..].to_vec());
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_of_params() {
        let a = vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ];
        let b = vec![
            ("a".to_string(), 4),
            ("b".to_string(), 5),
            ("c".to_string(), 6),
        ];
        let res = max_of_params(&a, &b);
        assert_eq!(
            res,
            vec![
                ("a".to_string(), 4),
                ("b".to_string(), 5),
                ("c".to_string(), 6)
            ]
        );
    }
}
