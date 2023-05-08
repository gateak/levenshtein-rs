Levenshtein Distance Calculator

This program calculates the Levenshtein distance between two input strings. The Levenshtein distance, also known as edit distance, is the minimum number of single-character edits (i.e., insertions, deletions, and substitutions) required to transform one string into another.

The program takes two strings as input and outputs their Levenshtein distance. The input strings can be provided as command line arguments or entered interactively at runtime.
Usage

To run the program with command line arguments, execute:

shell

$ ./levenshtein_distance string1 string2

where string1 and string2 are the two strings for which you want to calculate the Levenshtein distance.

To run the program interactively, execute:

shell

$ ./levenshtein_distance

and enter the two strings when prompted.
Example

shell

$ ./levenshtein_distance kitten sitting
3

In this example, the Levenshtein distance between the strings "kitten" and "sitting" is 3.
Implementation

The program implements the Levenshtein distance algorithm using a dynamic programming approach. The algorithm builds a matrix of distances between prefixes of the two input strings, and uses this matrix to compute the distance between the full strings.

The program is implemented in Rust, using standard library modules for command line argument parsing and input/output operations.
