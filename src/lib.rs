pub enum Change {
    Delete(std::ops::Range<usize>),
    Replace(std::ops::Range<usize>, String),
}

pub trait Spellchecker {
    fn check(&self, input: &str) -> Vec<Change>;
}

pub struct NoopSpellchecker;

impl Spellchecker for NoopSpellchecker {
    fn check(&self, input: &str) -> Vec<Change> {
        vec![]
    }
}

struct AntispaceChecker;

impl Spellchecker for AntispaceChecker {
    fn check(&self, input: &str) -> Vec<Change> {
        input
            .match_indices(" ")
            .map(|(index, space)| Change::Delete(index..index + space.len()))
            .collect()
    }
}

fn apply_change(string: &mut String, change: Change) {
    // TODO: implement
}

pub fn spellcheck<T: Spellchecker>(input: &str, spellchecker: T) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}

pub fn spellcheck_dyn(input: &str, spellchecker: &dyn Spellchecker) -> String {
    let mut result = input.to_owned();
    for change in spellchecker.check(input) {
        apply_change(&mut result, change);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "Hello, is it me you're looking for?";
        let result = spellcheck(text, NoopSpellchecker);
        assert_eq!(text, result);

    }

    #[test]
    fn it_works_dyn() {
        let text = "Hello, is it me you're looking for?";
        let result = spellcheck_dyn(text, &NoopSpellchecker);
        assert_eq!(text, result);

        let spellcheckers: Vec<&dyn Spellchecker> = vec![&NoopSpellchecker, &AntispaceChecker];
        for sp in spellcheckers {
            spellcheck_dyn(text, sp);
        }
    }
}