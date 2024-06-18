# WordFormatter

WordFormatter is a Command Line Interface (CLI) tool that takes an input file containing words separated by newline (`\n`) characters and outputs the words formatted in a file, where each word is enclosed in double quotes, followed by a comma and a newline.

## Features

- Reads an input file with words separated by newline (`\n`).
- Converts each word to the format `"word",\n`.
- Writes the output to a specified file.

## Usage

```
wordformatter -i input.txt -o output.txt
```

## Exaple

    input:
        HELLO
        WORLD
        ...
    output:
        "HELLO",
        "WORLD",
