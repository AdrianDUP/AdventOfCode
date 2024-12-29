package main

import "slices"

func smallest(number_one int, number_two int, number_three int) int {
    number_list := []int{number_one, number_two, number_three,}

    slices.Sort(number_list)

    return number_list[0]
}
