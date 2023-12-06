use std::{
    cmp::min, env, fs::read_to_string, ops::Range, process, str::FromStr
};

struct Almanac {
    pub seeds: Vec<Range<u64>>,
    categories: Vec<Category>
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum CategoryName {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemp,
    TempToHumidity,
    HumidityToLocation
}

impl FromStr for CategoryName {
    type Err = ();

    fn from_str(input: &str) -> Result<CategoryName, Self::Err> {
        match input {
            "seed-to-soil" => Ok(CategoryName::SeedToSoil),
            "soil-to-fertilizer" => Ok(CategoryName::SoilToFertilizer),
            "fertilizer-to-water" => Ok(CategoryName::FertilizerToWater),
            "water-to-light" => Ok(CategoryName::WaterToLight),
            "light-to-temperature" => Ok(CategoryName::LightToTemp),
            "temperature-to-humidity" => Ok(CategoryName::TempToHumidity),
            "humidity-to-location" => Ok(CategoryName::HumidityToLocation),
            _ => Err(())
        }
    }
}

struct Category {
    _name: CategoryName,
    maps: Vec<CategoryMap>
}

struct CategoryMap {
    src: Range<u64>,
    dst: Range<u64>
}

impl Almanac {
    fn load(file_data: String) -> Self {
        let mut lines = file_data.lines();

        let mut seeds: Vec<Range<u64>> = vec![];
        let mut categories: Vec<Category> = vec![];

        loop {
            match lines.next() {
                Some(line) if line.starts_with("seeds:") => {
                    if env::var("PART2").is_ok() {
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
                        .parse::<CategoryName>()
                        .expect("category string to 'Category' enum");

                    let mut category_maps: Vec<CategoryMap> = vec![];

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

                        category_maps.push(map);
                    }

                    categories.push(Category {
                        _name: category,
                        maps: category_maps
                    });
                },
                Some(_) => {
                    // ruh-roh, line not handled... unless intended "of course"
                },
                None => break
            }
        }

        Almanac {
            seeds,
            categories
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
        let mut val = seed;

        for category in &self.categories {
            val = Almanac::map_value(category, val);
        }

        val
    }

    fn map_value(category: &Category, val:u64) -> u64 {
        for map in &category.maps {
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
