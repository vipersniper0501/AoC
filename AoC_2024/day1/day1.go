package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func partition(list []int, low, high int) (index int) {

    pivot := list[high]
    index = low
    for j := low; j < high; j++ {
        if list[j] < pivot {
            list[index], list[j] = list[j], list[index]
            index++
        }
    }

    list[index], list[high] = list[high], list[index]

    return
}

func quicksort(list []int, low int, high int) {

    if low < high {
        p := partition(list, low, high)
        quicksort(list, low, p-1)
        quicksort(list, p+1, high)
    }

}


func parse_input() (list1 []int, list2 []int) {

    f, err := os.Open("input")
    if err != nil {
        fmt.Println("File io error: ", err)
    }
    defer f.Close()

    scanner := bufio.NewScanner(f)

    for scanner.Scan() {
        line := scanner.Text()
        values := strings.Fields(line)
        v1, err := strconv.Atoi(values[0])
        if err != nil {
            fmt.Println("Failed to convert string to int")
            return
        }
        list1 = append(list1, v1)
        v2, err := strconv.Atoi(values[1])
        if err != nil {
            fmt.Println("Failed to convert string to int")
            return
        }
        list2 = append(list2, v2)
    }

    if err := scanner.Err(); err != nil {
        fmt.Println("File io error: ", err)
    }

    return
}

func part1(list1 []int, list2 []int) {

    quicksort(list1, 0, len(list1) - 1)
    quicksort(list2, 0, len(list2) - 1)

    sum := 0
    for i := 0; i < len(list1); i++ {
        val := list1[i] - list2[i]
        if val < 0 {
            val = -val
        }

        sum += val
    }

    fmt.Println("Part 1 Solution: Sum = ", sum)

}

func part2(list1 []int, list2 []int) {
    quicksort(list1, 0, len(list1) - 1)
    quicksort(list2, 0, len(list2) - 1)

    similarity_score := 0

    right_index := 0
    for left_index := 0; left_index < len(list1); left_index++ {
        left_value := list1[left_index]
        counter := 0
        for j := right_index; j < len(list2); j++ {
            if left_value == list2[j] {
                counter++
            } else if left_value < list2[j] {
                right_index = j
                break
            }

        }
        similarity_score += left_value * counter
    }


    fmt.Println("Part 2 Solution: Similarity Score = ", similarity_score)

}

func main() {

    list1, list2 := parse_input()

    part1(list1, list2)
    part2(list1, list2)

}
