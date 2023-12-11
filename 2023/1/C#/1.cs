// See https://aka.ms/new-console-template for more information

string[] input = File.ReadAllLines("./../../../../input.txt");

foreach (string line in input) {
    string replaceWordedNumbers = line.Replace("one", "o1e")
		.Replace("two", "t2o")
		.Replace("three", "t3e")
		.Replace("four", "f4")
		.Replace("five", "f5e")
		.Replace("six", "s6")
		.Replace("seven", "s7n")
		.Replace("eight", "e8t")
		.Replace("nine", "n9");
    
	Console.WriteLine(replaceWordedNumbers);
}

Console.ReadLine();