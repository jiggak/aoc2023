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
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    T = 10,
    #[cfg(feature="wildcard")]
    J = 1,
    #[cfg(not(feature="wildcard"))]
    J = 11,
    Q = 12,
    K = 13,
    A = 14
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

        let type_ = if cfg!(feature="wildcard") {
            get_hand_type_wildcard(&cards)
        } else {
            get_hand_type(&cards)
        };

        Hand { cards, bid, type_ }
    }

    fn load(file_content: &str) -> Vec<Self> {
        file_content.lines()
            .map(|l| Hand::parse(l))
            .collect()
    }
}

fn get_hand_type_wildcard(cards: &Vec<Card>) -> HandType {
    if cards.contains(&Card::J) {
        let all_cards = [
            Card::Two, Card::Three, Card::Four, Card::Five, Card::Six,
            Card::Seven, Card::Eight, Card::Nine, Card::T, Card::Q, Card::K,
            Card::A
        ];

        let mut possible_hands: Vec<HandType> = vec![];

        for card in all_cards {
            let cards: Vec<_> = cards.iter()
                .map(|c| match c {
                    Card::J => card,
                    c => *c
                })
                .collect();

            possible_hands.push(get_hand_type(&cards));
        }

        possible_hands.sort(); // sort weakest to strongest

        *possible_hands.last().unwrap() // take strongest possible
    } else {
        get_hand_type(&cards)
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

    let input_file = match args.next() {
        Some(f) => f,
        None => {
            print_usage_exit(&cmd_name);
        }
    };

    let file_content = fs::read_to_string(input_file)
        .expect("input file should exist and be text file");

    let total = total_hand_winnings(&file_content);

    println!("{total}");
}

fn print_usage_exit(me: &str) -> ! {
    println!("{me} input.txt");
    process::exit(1)
}

fn total_hand_winnings(file_data: &str) -> usize {
    let mut hands = Hand::load(file_data);

    hands.sort(); // sort weakest to strongest

    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += (i+1) * hand.bid as usize;
    }

    total
}
