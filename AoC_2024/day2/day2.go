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
        if is_safe(reports[i]) {
            safe_reports++
        }
    }

    fmt.Println("Solution to Part 1: Safe reports = ", safe_reports)

}

func remove(slice []int, s int) []int {
    return append(slice[:s], slice[s+1:]...)
}

func is_safe(report []int) bool {
    decreasing := true
    increasing := true

    for j := 1; j < len(report); j++ {
        l_val := report[j-1]
        c_val := report[j]
        diff := c_val - l_val
        if diff < 0 {
            diff = -diff
        }
        if diff > 3 || diff < 1 {
            return false
        }


        if l_val < c_val {
            decreasing = false
        }
        if l_val > c_val {
            increasing = false
        }
    }

    return decreasing || increasing
}

func part2(reports [][]int) {
    safe_reports := 0

    for i := 0; i < len(reports); i++ {
        if is_safe(reports[i]) {
            safe_reports++
        } else {
            for j := 0; j < len(reports[i]); j++ {
                copied := make([]int, len(reports[i]))
                copy(copied, reports[i])
                copied = remove(copied, j)
                if is_safe(copied) {
                    safe_reports++
                    break
                }
            }
        }
    }

    fmt.Println("Solution to Part 2: Safe reports = ", safe_reports)

}

func main() {

    reports := parse_input()
    part1(reports)
    part2(reports)

}
