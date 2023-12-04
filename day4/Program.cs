class Program
{
    static int Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.Error.WriteLine($"Program.Main [input.txt]");
            return 1;
        }

        var part2 = Environment.GetEnvironmentVariable("PART2") != null;
        var cards = ReadCards(args[0]);

        int total;

        if (part2)
        {
            var cardsList = cards.ToList();
            total = ApplyCloneRules(cardsList, cardsList, cardsList.Count);
        }
        else
        {
            total = TotalCardPoints(cards);
        }

        Console.WriteLine(total);

        return 0;
    }

    static IEnumerable<Card> ReadCards(string filePath)
    {
        using (var reader = File.OpenText(filePath))
        {
            string? line;
            while ((line = reader.ReadLine()) != null)
            {
                yield return Card.Parse(line);
            }
        }
    }

    static int TotalCardPoints(IEnumerable<Card> cards)
    {
        int total = 0;

        foreach (var card in cards)
        {
            total += card.Points();
        }

        return total;
    }

    static int ApplyCloneRules(IEnumerable<Card> originalCards, List<Card> searchCards, int count)
    {
        var clones = new List<Card>();

        foreach (var card in searchCards)
        {
            var matchCount = card.MatchCount();
            clones.AddRange(originalCards.Skip(card.Id).Take(matchCount));
        }

        if (clones.Count > 0)
        {
            return ApplyCloneRules(originalCards, clones, count + clones.Count);
        }
        else
        {
            return count;
        }
    }
}

class Card
{
    public int Id { get; set; }

    public int[] WinningNumbers { get; private set; }

    public int[] CardNumbers { get; private set; }

    public int MatchCount()
    {
        return CardNumbers.Where(c => WinningNumbers.Contains(c)).Count();
    }

    public int Points()
    {
        var matchCount = MatchCount();
        if (matchCount == 0)
            return 0;

        return (int)Math.Pow(2, matchCount - 1);
    }

    public static Card Parse(string line)
    {
        var splitOpts = StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries;

        var parts = line.Split(':');
        var cardPrefix = parts[0];
        var cardNumbers = parts[1];

        var cardId = int.Parse(cardPrefix.Split(' ', splitOpts)[1]);

        parts = cardNumbers.Split('|', splitOpts);

        var numbers1 = parts[0].Split(' ', splitOpts)
            .Select(int.Parse).ToArray();

        var numbers2 = parts[1].Split(' ', splitOpts)
            .Select(int.Parse).ToArray();

        return new Card()
        {
            Id = cardId,
            WinningNumbers = numbers1,
            CardNumbers = numbers2
        };
    }

    public override string ToString()
    {
        return $"Card {Id}: {string.Join(',', WinningNumbers)} | {string.Join(',', CardNumbers)}";
    }
}
