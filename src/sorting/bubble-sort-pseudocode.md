Bubble Sort Pseudocode 

function(collection):
    let n = collection length

    if n <=1 {
        return collection
    }
 

    for i from  0 to n-1:
        let swapped = false    //track if any swaps occur in the iteration

        for j from 1 to n-i-1:  //compare adjacent elements
            if collection[j-1] > collection[j]
            swap(collection[j-1],collection[j) //swap elements
            swapped = true

    if swapped == false:
        break   // no swaps array is sorted
    
    return collection //return sorted array