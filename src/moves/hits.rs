use super::damage_move::Options;

#[derive(Debug)]
pub enum Hits {
    Count(usize),
    Compound {
        probability: usize,
        hits_max: usize,
        simplified_hits: usize,
        ikasama_hits: usize,
        inc_power: bool,
    },
    Random {
        hits_min: usize,
        hits_max: usize,
        simplified_hits: usize,
        ikasama_hits: usize,
    },
}

impl Hits {
    pub fn new(o: &Options) -> Result<Hits, String> {
        if let Some(probability) = o.compound {
            if o.hits.is_some() {
                Err("hits is illegal in this context")?
            }
            let hits_max = o.hits_max.ok_or("Compound must have hit_max")?;
            //トリプルアクセル、ねずみざんが念頭に置かれている
            let simplified_hits = if hits_max <= 3 {
                hits_max
            } else {
                (3 + hits_max) / 2
            };
            let ikasama_hits = if hits_max <= 4 {
                hits_max
            } else {
                (4 + hits_max) / 2
            };
            return Ok(Hits::Compound {
                probability,
                hits_max,
                simplified_hits,
                ikasama_hits,
                inc_power: o.inc_power.is_some(),
            });
        }
        if let Some(hits_min) = o.hits_min {
            if o.hits.is_some() {
                Err("hits is illegal in this context")?
            }
            let hits_max = o.hits_max.ok_or("hit_max must be paired with hit_min")?;
            let simplified_hits = (hits_min + hits_max) / 2;
            let ikasama_hits = if hits_max <= 4 { 4 } else { (4 + hits_max) / 2 };
            return Ok(Hits::Random {
                hits_min,
                hits_max,
                simplified_hits,
                ikasama_hits,
            });
        }
        if o.hits_max.is_some() {
            Err("hit_max is illegal in this context")?
        }
        Ok(Hits::Count(
            o.hits.ok_or("hits must exist in this context")?,
        ))
    }

    pub fn hits(&self, ikasama: bool) -> HitsResult {
        match self {
            Hits::Count(c) => HitsResult::new(*c, false),
            Hits::Random {
                hits_min: hit_min,
                hits_max: hit_max,
                simplified_hits,
                ikasama_hits,
            } => {
                if ikasama {
                    HitsResult::new(ikasama_hits, false)
                } else {
                    HitsResult::new(simplified_hits, false)
                }
            }
            Hits::Compound {
                probability,
                hits_max: hit_max,
                simplified_hits,
                ikasama_hits,
                inc_power,
            } => {
                if ikasama {
                    HitsResult::new(ikasama_hits, inc_power)
                } else {
                    HitsResult::new(simplified_hits, inc_power)
                }
            }
        }
    }
}

pub struct HitsResult {
    pub count: usize,
    pub inc_power: bool,
}

impl HitsResult {
    pub fn new(count: usize, inc_power: bool) -> Self {
        Self { count, inc_power }
    }
}
