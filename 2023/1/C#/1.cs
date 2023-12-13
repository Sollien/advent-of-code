using System.Text.RegularExpressions;

string[] input = File.ReadAllLines("./../../../../input.txt");
int[] sumList;

input = input.Select(s => s
	.Replace("one", "o1e")
    .Replace("two", "t2o")
    .Replace("three", "t3e")
    .Replace("four", "f4")
    .Replace("five", "f5e")
    .Replace("six", "s6")
    .Replace("seven", "s7n")
    .Replace("eight", "e8t")
    .Replace("nine", "n9")).ToArray();

for (int i = 0; i < input.Length; i++) {
    MatchCollection numbersInString = new Regex(@"\b[0-9]\w*\b").Matches(input[i]);

    foreach (var number in numbersInString)
    {
        Console.WriteLine(number);
    }
}


Console.ReadLine();