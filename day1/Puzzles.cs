List<int> elfCals = new();
int currCals = 0;

foreach (var line in File.ReadAllLines("elfCals.txt")) {
    if (int.TryParse(line, out int value)) {
        currCals += value;
    } else {
        elfCals.Add(currCals);
        currCals = 0;
    }
}

elfCals.Sort((a, b) => b.CompareTo(a));
int total = elfCals[0] + elfCals[1] + elfCals[2];
Console.WriteLine($"Answer1: {elfCals[0]}");
Console.WriteLine($"Answer2: {total}");
