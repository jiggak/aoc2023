use std::{
    cmp::min, collections::HashMap, env, fs::read_to_string, ops::Range, process,
    str::FromStr
};

struct Almanac {
    pub seeds: Vec<Range<u64>>,
    category_maps: HashMap<Category, Vec<CategoryMap>>
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Category {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumidity,
    HumidityToLocation
}

impl FromStr for Category {
    type Err = ();

    fn from_str(input: &str) -> Result<Category, Self::Err> {
        match input {
            "seed-to-soil" => Ok(Category::SeedToSoil),
            "soil-to-fertilizer" => Ok(Category::SoilToFertilizer),
            "fertilizer-to-water" => Ok(Category::FertilizerToWater),
            "water-to-light" => Ok(Category::WaterToLight),
            "light-to-temperature" => Ok(Category::LightToTemp),
            "temperature-to-humidity" => Ok(Category::TempToHumidity),
            "humidity-to-location" => Ok(Category::HumidityToLocation),
            _ => Err(())
        }
    }
}

struct CategoryMap {
    src: Range<u64>,
    dst: Range<u64>
}

impl Almanac {
    fn load(file_data: String) -> Self {
        let mut lines = file_data.lines();

        let mut seeds: Vec<Range<u64>> = vec![];
        let mut category_maps = HashMap::new();

        loop {
            match lines.next() {
                Some(line) if line.starts_with("seeds:") => {
                    if option_env!("PART2").is_some() {
                        let parts: Vec<_> = line.replace("seeds: ", "")
                            .split(" ")
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();

                        seeds = parts.chunks(2)
                            .map(|c| c[0]..(c[0] + c[1]))
                            .collect();
                    } else {
                        seeds = line.replace("seeds: ", "")
                            .split(" ")
                            .map(|s| s.parse::<u64>().unwrap())
                            .map(|s| s..s+1)
                            .collect();
                    }
                },
                Some(line) if line.ends_with("map:") => {
                    let category = line.replace(" map:", "")
                        .parse::<Category>()
                        .expect("category string to 'Category' enum");

                    loop {
                        let line = match lines.next() {
                            Some(line) if !line.is_empty() => line,
                            _ => break
                        };

                        let mut parts = line.split(" ");
                        let (dst_start, src_start, len) = (
                            parts.next().unwrap().parse::<u64>().unwrap(),
                            parts.next().unwrap().parse::<u64>().unwrap(),
                            parts.next().unwrap().parse::<u64>().unwrap()
                        );

                        let map = CategoryMap {
                            src: src_start..(src_start + len),
                            dst: dst_start..(dst_start + len)
                        };

                        let mapping = category_maps.entry(category)
                            .or_insert_with(|| vec![]);

                        mapping.push(map);
                    }
                },
                Some(_) => {
                    // ruh-roh, line not handled... unless intended "of cource"
                },
                None => break
            }
        }

        Almanac {
            seeds,
            category_maps
        }
    }

    fn find_lowest_location(&self) -> u64 {
        let mut loc = u64::MAX;

        for seed_range in self.seeds.iter() {
            for seed in seed_range.clone() {
                loc = min(loc, self.seed_to_location(seed));
            }
        }

        loc
    }

    fn seed_to_location(&self, seed: u64) -> u64 {
        let categories = [
            Category::SeedToSoil,
            Category::SoilToFertilizer,
            Category::FertilizerToWater,
            Category::WaterToLight,
            Category::LightToTemp,
            Category::TempToHumidity,
            Category::HumidityToLocation
        ];

        let mut val = seed;

        for category in categories {
            val = self.map_value(category, val);
        }

        val
    }

    fn map_value(&self, category:Category, val:u64) -> u64 {
        let maps = self.category_maps.get(&category).unwrap();

        for map in maps {
            if map.src.contains(&val) {
                return map.dst.start + (val - map.src.start);
            }
        }

        val
    }
}

fn main() {
    let mut args = env::args();

    // first arg is command name
    let cmd_name = args.next().unwrap();

    let input_file = match args.next() {
        Some(f) => f,
        None => {
            println!("{cmd_name} [input.txt]");
            process::exit(1);
        }
    };

    let file_content = read_to_string(input_file)
        .expect("input file should exist and be text file");

    let almanac = Almanac::load(file_content);
    let min_loc = almanac.find_lowest_location();

    println!("{min_loc}");
}
