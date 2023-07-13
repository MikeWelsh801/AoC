internal class Program
{
    public static void Main(string[] args)
    {
        Dictionary<string, string> translate = new() {
            {"A", "rock"},
            {"B", "paper"},
            {"C", "scissors"},
            {"X", "rock"},
            {"Y", "paper"},
            {"Z", "scissors"}
        };
        Dictionary<string, string> realTranslate = new() {
            {"X", "lose"},
            {"Y", "draw"},
            {"Z", "win"}
        };

        int round1Total = 0;
        int round2Total = 0;
        foreach (string line in File.ReadAllLines("strategy.txt"))
        {
            string[] plays = line.Split();
            string opPlay = translate[plays[0]];
            string myPlay = translate[plays[1]];
            int myScore = GetMyPlayScore(myPlay) + GetResultScore(myPlay, opPlay);
            round1Total += myScore;

            string result = realTranslate[plays[1]];
            myPlay = GetMyPlay(result, opPlay);
            myScore = GetMyPlayScore(myPlay) + GetResultScore(myPlay, opPlay);
            round2Total += myScore;
        }
        Console.WriteLine($"Answer 1: {round1Total}");
        Console.WriteLine($"Answer 2: {round2Total}");
    }

    private static string GetMyPlay(string result, string opPlay)
    {
        if (opPlay == "rock")
        {
            return result switch
            {
                "win" => "paper",
                "lose" => "scissors",
                "draw" => "rock",
                _ => ""
            };
        }
        else if (opPlay == "paper")
        {
            return result switch
            {
                "win" => "scissors",
                "lose" => "rock",
                "draw" => "paper",
                _ => ""
            };
        }
        else
        {
            return result switch
            {
                "win" => "rock",
                "lose" => "paper",
                "draw" => "scissors",
                _ => ""
            };
        }
    }

    private static int GetResultScore(string myPlay, string opPlay)
    {
        if (myPlay == "rock")
        {
            return opPlay switch
            {
                "rock" => 3,
                "paper" => 0,
                "scissors" => 6,
                _ => int.MinValue
            };
        }
        else if (myPlay == "paper")
        {
            return opPlay switch
            {
                "rock" => 6,
                "paper" => 3,
                "scissors" => 0,
                _ => int.MinValue
            };
        }
        else
        {
            return opPlay switch
            {
                "rock" => 0,
                "paper" => 6,
                "scissors" => 3,
                _ => int.MinValue
            };
        }
    }

    private static int GetMyPlayScore(string type)
    {
        return type switch
        {
            "rock" => 1,
            "paper" => 2,
            "scissors" => 3,
            _ => int.MinValue
        };
    }
}
