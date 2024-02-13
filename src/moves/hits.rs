use super::damage_move::Options;

#[derive(Debug)]
pub enum Hits {
    Count(usize),
    Compound {
        probability: usize,
        hit_max: usize,
        simplified_hits: usize,
        ikasama_hits: usize,
        inc_power: bool,
    },
    Random {
        hit_min: usize,
        hit_max: usize,
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
            let hit_max = o.hit_max.ok_or("Compound must have hit_max")?;
            //トリプルアクセル、ねずみざんが念頭に置かれている
            let simplified_hits = if hit_max <= 3 {
                hit_max
            } else {
                (3 + hit_max) / 2
            };
            let ikasama_hits = if hit_max <= 4 {
                hit_max
            } else {
                (4 + hit_max) / 2
            };
            return Ok(Hits::Compound {
                probability,
                hit_max,
                simplified_hits,
                ikasama_hits,
                inc_power: o.inc_power.is_some(),
            });
        }
        if let Some(hit_min) = o.hit_min {
            if o.hits.is_some() {
                Err("hits is illegal in this context")?
            }
            let hit_max = o.hit_max.ok_or("hit_max must be paired with hit_min")?;
            let simplified_hits = (hit_min + hit_max) / 2;
            let ikasama_hits = if hit_max <= 4 { 4 } else { (4 + hit_max) / 2 };
            return Ok(Hits::Random {
                hit_min,
                hit_max,
                simplified_hits,
                ikasama_hits,
            });
        }
        if o.hit_max.is_some() {
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
                hit_min,
                hit_max,
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
                hit_max,
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
