use std::io;

// Example values:
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

pub type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&[u8]> for Color {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Color> {
        match value {
            b"red" => Ok(Color::Red),
            b"green" => Ok(Color::Green),
            b"blue" => Ok(Color::Blue),
            _v => Err("Unknown color literal {_v}".into()),
        }
    }
}
#[derive(Debug)]
struct Subset {
    red: u8,
    green: u8,
    blue: u8,
}

impl Subset {
    /// Extracts next subset and remaining part from provided byte slice.
    /// Slice must be of the "right part", i.e. result of splitting game line by ":"
    pub fn parse_next(str: &[u8]) -> (&[u8], Option<&[u8]>) {
        if let Some((sep_idx, _)) = str
            .iter()
            .enumerate()
            .find(|(_, c)| **c == b';' || **c == b'\n')
        {
            return (&str[..sep_idx], Some(&str[sep_idx + 1..]));
        }
        (str, None)
    }

    pub fn possible_with(&self, max_subset: &Subset) -> bool {
        self.red <= max_subset.red && self.blue <= max_subset.blue && self.green <= max_subset.green
    }
}

impl TryFrom<&[u8]> for Subset {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Subset> {
        let (mut red, mut green, mut blue) = (0u8, 0u8, 0u8);
        let tokens: Vec<&[u8]> = value
            .split(|&c| c == b' ' || c == b',')
            .filter(|v| !v.is_empty())
            .collect();

        for i in (0..tokens.len() - 1).step_by(2) {
            let (raw_v, raw_c) = (tokens[i], tokens[i + 1]);
            // eprintln!("v: '{}'", std::str::from_utf8(raw_v).unwrap());
            let v: u8 = std::str::from_utf8(raw_v)?.parse()?;
            match Color::try_from(raw_c)? {
                Color::Red => red = v,
                Color::Green => green = v,
                Color::Blue => blue = v,
            }
        }
        Ok(Self { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    id: u8,
    subsets: Vec<Subset>,
}

impl Game {
    pub fn parse_id(str: &[u8]) -> Result<u8> {
        Ok(std::str::from_utf8(&str[5..])?.parse()?)
    }
}

impl TryFrom<&[u8]> for Game {
    type Error = Error;

    fn try_from(value: &[u8]) -> std::prelude::v1::Result<Self, Self::Error> {
        let [l_part, r_part]: [&[u8]; 2] = value
            .split(|c| *c == b':')
            .collect::<Vec<&[u8]>>()
            .try_into()
            .map_err(|_| "Could not split line into two parts!")?;
        let id = Game::parse_id(l_part)?;
        let mut subsets: Vec<Subset> = Vec::with_capacity(16);
        let mut remainder = r_part;
        loop {
            let (raw_subset, mby_next_remainder) = Subset::parse_next(remainder);
            subsets.push(Subset::try_from(raw_subset)?);

            if let Some(next_remainder) = mby_next_remainder {
                remainder = next_remainder;
            } else {
                break;
            }
        }

        Ok(Self { id, subsets })
    }
}

const MAX_SUBSET: Subset = Subset {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() -> Result<()> {
    let lines = io::stdin().lines();
    let mut game_ids: Vec<u8> = Vec::new();
    for line in lines {
        let game = Game::try_from(line?.as_bytes())?;
        if game
            .subsets
            .iter()
            .all(|subset| subset.possible_with(&MAX_SUBSET))
        {
            game_ids.push(game.id);
        }
    }
    print!("{}", game_ids.iter().map(|id| *id as u16).sum::<u16>());
    Ok(())
}
