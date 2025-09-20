package main

import (
	"bufio"
	"fmt"
	"os"
)

func parser() (memory []string) {
    
    f, err := os.Open("input")
    if err != nil {
        fmt.Println("File io error: ", err)
        return
    }
    defer f.Close()

    scanner := bufio.NewScanner(f)

    for scanner.Scan() {
        line := scanner.Text()
        _ = line
    }


    return
}

func main() {

    memory := parser()
    _ = memory

}
