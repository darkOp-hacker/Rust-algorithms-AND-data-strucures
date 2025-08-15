MERGE SORT PSEUDOCODE

function merge_sort(collection):
    
    if collection_length <= 1 :
        return collection //already sorted

    let middle_index = collection_length / 2
    let left_half = elements from start to middle
    let right_half = elemets from middle to end

    let sorted_left = merge_sort(left_half)
    let sorted_right = merge_sort(right_half)
    
    return merge(sorted_left,sorted_right)


function merge(left,right):
    
    let result = new empty collection
    let i = 0 
    let j = 0
    let k = 0
    
    while i < left_length AND j < right_length:
    
        if left[i] <= right[j]:
            result[k] = left[i]
            k += 1
            i += 1

        else:
            result[k] = right[j]
            k +=1
            j += 1


    
    while i < left_length:
        result[k] = left[i]
        i += 1
        k += 1

    while j < right_length:
        result[k] = right[j]
        j += 1
        k += 1


    return result


            
    
    