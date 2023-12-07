use std::{
    {env, fs, process},
    cmp::Ordering, collections::HashSet, str::FromStr
};

/// Hand types ordered weakest to strongest
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind
}

/// Cards orderd weakest to strongest
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Card {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, T, J, Q, K, A
}

impl FromStr for Card {
    type Err = ();

    fn from_str(input: &str) -> Result<Card, Self::Err> {
        match input {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::T),
            "J" => Ok(Card::J),
            "Q" => Ok(Card::Q),
            "K" => Ok(Card::K),
            "A" => Ok(Card::A),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    type_: HandType
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ord = self.type_.cmp(&other.type_);
        if type_ord.is_eq() {
            self.cards.cmp(&other.cards)
        } else {
            type_ord
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(' ');

        let cards = parts.next()
            .expect("card characters")
            .chars()
            .map(|c| Card::from_str(c.to_string().as_str()).unwrap())
            .collect();

        let bid = parts.next()
            .expect("card bid")
            .parse::<u32>().expect("card bid as int");

        let type_ = get_hand_type(&cards);

        Hand {
            cards,
            bid,
            type_
        }
    }

    fn load(file_content: &str) -> Vec<Self> {
        file_content.lines()
            .map(|l| Hand::parse(l))
            .collect()
    }
}

fn get_hand_type(cards: &Vec<Card>) -> HandType {
    let set: HashSet<&Card> = HashSet::from_iter(cards);

    if set.len() == 1 {
        HandType::FiveOfKind
    } else if set.len() == 2 {
        // four of a kind or full house

        let counts: Vec<_> = set.iter()
            .map(|c| cards.iter().filter(|x| x == c).count())
            .collect();

        if counts.contains(&4) {
            HandType::FourOfKind
        } else {
            HandType::FullHouse
        }
    } else if set.len() == 3 {
        // three of a kind or two pair

        let counts: Vec<_> = set.iter()
            .map(|c| cards.iter().filter(|x| x == c).count())
            .collect();

        if counts.contains(&3) {
            HandType::ThreeOfKind
        } else {
            HandType::TwoPair
        }
    } else if set.len() == 4 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}


fn main() {
    let mut args = env::args();

    // first arg is command name
    let cmd_name = args.next().unwrap();

    let mut run_part2 = false;
    let input_file = match args.next() {
        Some(a) if a == "-p2" => {
            run_part2 = true;
            args.next()
        },
        Some(a) => Some(a),
        None => None
    }.unwrap_or_else(|| print_usage_exit(&cmd_name));

    let file_content = fs::read_to_string(input_file)
        .expect("input file should exist and be text file");

    let total = if run_part2 {
        part2(&file_content)
    } else {
        part1(&file_content)
    };

    println!("{total}");
}

fn print_usage_exit(me: &str) -> ! {
    println!("{me} [-p2] input.txt");
    process::exit(1)
}

fn part1(file_data: &str) -> usize {
    let mut hands = Hand::load(file_data);

    hands.sort(); // sort weakest to strongest
    // hands.reverse(); // strongest to weakest

    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += (i+1) * hand.bid as usize;
    }

    total
}

fn part2(_file_data: &str) -> usize {
    0
}
