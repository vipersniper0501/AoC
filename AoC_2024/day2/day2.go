package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse_input() (reports [][]int){
    f, err := os.Open("input")
    if err != nil {
        fmt.Println("File io error: ", err)
        return
    }
    defer f.Close()

    scanner := bufio.NewScanner(f)

    for scanner.Scan() {
        line := scanner.Text()

        // Line by line parsing here
        values := strings.Fields(line)

        var int_values []int
        for i := 0; i < len(values); i++ {
            v, err := strconv.Atoi(values[i])
            if err != nil {
                fmt.Println("Failed to convert string to int")
                return
            }

            int_values = append(int_values, v)
        }

        reports = append(reports, int_values)
    }

    if err := scanner.Err(); err != nil {
        fmt.Println("File io error: ", err)
        return
    }

    return
}

func part1(reports [][]int) {

    safe_reports := 0

    for i := 0; i < len(reports); i++ {
        var c_val int
        if len(reports[i]) < 2 {
            break
        }
        decreasing := false
        if reports[i][1] < reports[i][0] {
            decreasing = true
        }
        failed_report := false
        for j := 1; j < len(reports[i]); j++ {
            c_val = reports[i][j-1]
            n_val := reports[i][j]

            if n_val < c_val && decreasing {
                diff := n_val - c_val
                if diff < 0 {
                    diff = -diff
                }
                if diff > 3 || diff < 1 {
                    failed_report = true
                    break

                }

            } else if n_val > c_val && !decreasing {
                diff := n_val - c_val
                if diff < 0 {
                    diff = -diff
                }
                if diff > 3 || diff < 1 {
                    failed_report = true
                    break
                }

            } else {
                failed_report = true
                break
            }
        }
        if !failed_report {
            safe_reports++
        }
    }

    fmt.Println("Solution to Part 1: Safe reports = ", safe_reports)

}

func part2() {

}

func main() {

    reports := parse_input()
    part1(reports)

}
