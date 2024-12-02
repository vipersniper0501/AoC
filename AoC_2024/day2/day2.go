package main

import (
	"bufio"
	"fmt"
	"os"
)

func parse_input() {
    f, err := os.Open("input")
    if err != nil {
        fmt.Println("File io error: ", err)
        return
    }
    defer f.Close()

    scanner := bufio.NewScanner(f)

    for scanner.Scan() {
        line := scanner.Text()
        _  = line

        // Line by line parsing here


    }

    if err := scanner.Err(); err != nil {
        fmt.Println("File io error: ", err)
        return
    }

    return
}

func part1() {

}

func part2() {

}

func main() {

}
