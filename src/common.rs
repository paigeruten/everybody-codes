use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone)]
pub enum Part {
    One,
    Two,
    Three,
}

impl Part {
    pub fn default_input_path(self, quest_num: usize) -> String {
        let part_num: usize = self.into();
        format!("notes/q{:02}p{:02}", quest_num, part_num)
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let part_num: usize = (*self).into();
        write!(f, "{}", part_num)
    }
}

impl From<Part> for usize {
    fn from(part: Part) -> Self {
        match part {
            Part::One => 1,
            Part::Two => 2,
            Part::Three => 3,
        }
    }
}

impl TryFrom<usize> for Part {
    type Error = color_eyre::Report;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            3 => Ok(Part::Three),
            _ => Err(color_eyre::eyre::eyre!(
                "Part number must be between 1 and 3"
            )),
        }
    }
}

pub fn file_reader(path: &str) -> color_eyre::Result<impl BufRead> {
    Ok(BufReader::new(File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::Part;

    #[test]
    fn default_input_path_with_leading_zeroes() {
        assert_eq!("notes/q06p02", Part::Two.default_input_path(6));
    }

    #[test]
    fn default_input_path_without_leading_zeroes() {
        assert_eq!("notes/q10p03", Part::Three.default_input_path(10));
    }
}
