pub struct CodonsInfo<'a> {
    map: std::collections::HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        match self.map.get(&codon) {
            None => None,
            Some(v) => Some(*v),
        }
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut vec = vec![];
        for i in (3..rna.len() + 1).step_by(3) {
            let name = self.name_for(&rna[i-3..i]);
            match name {
                None => { return None; },
                Some(v) if v == "stop codon" => { return Some(vec); },
                Some(v) => { vec.push(v); },
            }
        }
        Some(vec)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo {
        map: pairs.into_iter().collect()
    }
}
