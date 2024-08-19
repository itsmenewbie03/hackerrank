package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func yeet(err error) {
	if err != nil {
		panic(err)
	}
}

func activityNotification(exp []int32, d int32) int32 {
	notifCount := 0
	rest := exp[d:]
	for idx, elem := range rest {
		var median float32
		median = 0
		hist := exp[idx : d+int32(idx)]
		fmt.Printf("(%v, %v) | ", idx, d+int32(idx))
		sort.Slice(hist, func(i, j int) bool {
			return hist[i] < hist[j]
		})
		hlen := len(hist)
		if hlen%2 == 0 {
			median = float32(hist[hlen/2-1]+hist[hlen/2]) / float32(2)
		} else {
			median = float32(hist[hlen/2])
		}
		var flag string
		flag = "gosucks"
		if hlen%2 == 0 {
			flag = "even"
		} else {
			flag = "odd"
		}
		fmt.Printf("(%v, %v) | ", hist[hlen/2-1], hist[hlen/2])
		fmt.Printf("elem: %v >= median*2: %v = %v | %v\n", elem, median*2, float32(elem) >= float32(median*2), flag)
		if float32(elem) >= float32((median * 2)) {
			notifCount += 1
		}
	}
	return int32(notifCount)
}

func parseArray(input string) []int32 {
	data := strings.Split(input, " ")
	out := make([]int32, len(data))
	for idx, elem := range data {
		parsedElem, err := strconv.ParseInt(elem, 10, 32)
		yeet(err)
		out[idx] = int32(parsedElem)
	}
	return out
}

func main() {
	data, err := os.ReadFile("./src/file.txt")
	yeet(err)
	lines := strings.Split(strings.TrimRight(string(data), "\n"), "\n")
	headers := lines[0]
	headersArr := strings.Split(headers, " ")
	d, err := strconv.ParseInt(headersArr[1], 10, 32)
	yeet(err)
	exp := parseArray(lines[1])
	notifCount := activityNotification(exp, int32(d))
	fmt.Printf("notifCount: %v\n", notifCount)
}
