# Word Count Cli

A simple program thaht counts occurences of words in text files and websites.

# Usage

The program setup file "app.properties" is located at the root of the project. There are 2 variables to set up to your liking **keywords** and **hop_count**.

## KEYWORDS

This variable is an array of words you want to find, you can add as many words as you want but it must have 1.

## HOP COUNT

This variable is a number that represents to "depth" that you would like to go.
Example: if you have a directory "FIRST" that has 2 text files and a directory "SECOND" inside it and **hop_count** of 1. The program is going to count only the words of the text files in the "FIRST" directory but skip the "SECOND" directory. If **hop_count** was two, the program would count all the text files in the "SECOND" directory.

## FILES

All of the files and directories located in the "example" directory will be counted by the program. The "example" directory is located at the root of the project.

## WEBSITES

All of the website links located in the "example_web/websites.txt" file will be counted by the program. The "example_web" directory is located at the root of the project.