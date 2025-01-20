# ATAD

## Solve me first

The solve_me_first function takes two integers a and b, and returns their sum.

## A very big sum
This function:
- First reads the total number of elements from the input.
- Next, it reads a space-separated list of numbers, which are converted to integers and stored in an array.
- The array of numbers is passed to the function that computes their sum.
- The function takes an array of large integers as input.
- It iterates through the array and calculates the total sum of all elements.
- The final sum is then returned as the result.
- Once the sum is calculated, it is written to an output file.
- The output file location is determined by an environment variable.

## Plus Minus
- The function first determines the total number of elements using arr.len(), converting it to f64 for accurate division.
- It then initializes three counters: positive, negative, and zero—to zero and iterates through the array using a match statement to classify each element.
- The function increments the positive counter for values greater than zero, the negative counter for values less than zero, and the zero counter for elements equal to zero.
- After counting, the function calculates the respective ratios by dividing each count by the total number of elements and prints the results.
- The main function handles input by reading the number of elements and parsing space-separated integers from standard input.
- It validates that the parsed elements match the given count before calling the plusMinus function.
- The program ensures correct output even with edge cases, such as arrays containing only positive, negative, or zero values.

## Subarray Division
This function
- Creates a variable count to track the number of valid ways to split the chocolate.
- It iterates through possible segments
- Loops through the chocolate bar to consider subarrays of length m.
- The range is 0..=s.len() - m to avoid exceeding array bounds.
- It extracts subarrays and calculates their sum.
- Checks if the sum equals d, and if so, it increments the count.
- After all the segments are processed, it returns the count of valid ways.
- It reads input values from the console,then parses the number of squares and stores the chocolate bar values in a vector.
- It calls the function with the parsed values and writes the result

## MiniMaxSum
The function
- Reads a single line of input containing space-separated integers. The input string is processed by trimming any trailing whitespace and splitting it into separate substrings.
- Each substring is then parsed into an integer using .parse::<i32>().unwrap(), and the resulting integers are collected into a vector.
- The miniMaxSum function first calculates the total sum of all elements in the input array by iterating through the vector using the .iter().map() method, converting each element to i64 to avoid potential overflow issues.
- Next, the function identifies the minimum and maximum values in the array using the .min() and .max() methods, which return references to the smallest and largest elements. These references are dereferenced using the \* operator to obtain their actual values.
- It then calculates the minimum possible sum by subtracting the largest element from the total sum, and the maximum possible sum by subtracting the smallest element.
- Finally, it prints the calculated minimum and maximum sums as space-separated integers.

## Divisible Sum Pairs
This function:
- It first reads input from standard input using io::stdin() and processes it line by line.
- The first line of input is split and parsed to retrieve the values of n (the number of elements in the array) and k (the divisor). The second line, containing the space-separated array elements, is parsed into a vector of integers.
- The divisibleSumPairs function calculates the number of valid pairs in the given array that satisfy the divisibility condition. It first initializes a vector, remainder_count, of size k to store the count of occurrences of each remainder when elements of the array are divided by k.
- The function then iterates over the array, calculating the remainder of each element when divided by k, ensureing the remainder is always non-negative.The complement that would sum to a multiple of k is calculated using (k - remainder) % k.
- For each element, the function adds the count of previously seen elements that complement the current remainder, forming a valid pair. After processing the element, it increments the count of the current remainder in the remainder_count vector. Finally, the function returns the total count of valid pairs.

## Find digits
This function:
- Reads input from standard input and locks it for efficient reading. The first input value is the number of test cases (t), which is read and parsed as an integer.
- For each test case, the integer n is read, parsed, and passed to the findDigits function to compute the result. 
- The findDigits function determines how many digits of a given integer n evenly divide n without leaving a remainder. It first converts the integer to a string using .to_string(), enabling easy access to each digit as a character.
- It initializes a counter to track the number of digits that divide n. It then iterates through each character in the string representation of the number. Each character is converted back to an integer using .to_digit(10).unwrap().
- To avoid division by zero errors, the function checks if the digit is not zero before performing the divisibility test. If the digit divides n evenly, the counter is incremented.
- Finally, the function returns the total count of digits that satisfy the divisibility condition.
- The function result is then written to a file specified by the OUTPUT_PATH environment variable.

## Append and Delete
- The function reads input values,it extracts the first line as string s, the second line as string t, and the third line as an integer k.
- The appendAndDelete function is called with the parsed values.
- It function starts by determining the length of the longest common prefix shared between s and t, iterates over both strings up to the length of the shorter string using .chars().nth(i) and increments common_length for each matching character.
- If a mismatch is found, the iteration stops.
- The number of deletions required is determined by subtracting the common prefix length from s's length.
- The number of additions required is determined by subtracting the common prefix length from t's length.
- The total operations required to convert s to t is the sum of deletions and additions.
- If the total number of operations required is greater than k, it is not possible to perform the transformation, No is returned.
- If the number of leftover operations (k - total_operations) is even, or if k is greater than or equal to the sum of the lengths of both strings, the function returns "Yes".
- Otherwise, the function returns "No".
- The result of the function is written to a file specified by the OUTPUT_PATH environment variable

## Equalize the array
- The function reads from standard input.
- The first line contains the integer n, which represents the number of elements in the array.
- The second line contains space-separated integers, which are parsed and collected into a vector.
- The equalizeArray function is called with the parsed vector.
- This function initializes a HashMap to store the frequency of each element in the array.
- It iterates over the array, using the .entry(elem).or_insert(0) method to update the count of each element in the map.
- The maximum frequency value in the frequency map is determined using .values().max().unwrap().
- This value represents the count of the most frequently occurring element in the array.
- The total number of elements in the array is subtracted by the highest frequency value to determine the number of deletions required to equalize the array.
- The function returns the computed number of deletions.
- The result is written to a file specified by the OUTPUT_PATH environment variable

## Picking Numbers
This function:
- It reads the input,the first line containing the integer n, which represents the number of elements in the array.
- The second line contains space-separated integers, which are split and parsed into a vector, and the parsed vector is passed to the pickingNumbers function to determine the result.
- In pickingNumbers function a frequency array count of size 101 is initialized to store occurrences of integers ranging from 0 to 100.
- The function iterates through the input vector a and increments the corresponding index in the count array for each number present.
- The function initializes max_length to track the longest valid subsequence found and it iterates and computes the sum of occurences for adjacent numbers.
- The result is written to a file specified by the OUTPUT_PATH environment variable.
- The function returns the length of the longest valid subsequence.

## Time conversion
This function:
- Read from standard input,the input string containing a single 12-hour formatted time string.
- The timeConversion function is called with the input string to compute the 24-hour formatted time.
- The AM/PM period is extracted from the last two characters of the string, representing whether the time is in the morning or afternoon.
- The hour part is extracted from the first two characters and parsed into an integer.
- The remaining portion, containing minutes and seconds, is stored for later formatting.
- If the period is "AM" and the hour is 12, it is converted to 00.
- If the period is "PM" and the hour is not 12, 12 hours are added to the current hour to convert to the 24-hour format,and if it is, it remains unchanged.
- The formatted 24-hour time string is created ensuring that the hour is represented with two digits.
- The result is written to a file specified by the OUTPUT_PATH environment variable.

## Camel case
- The function reads the input string from standard input, expecting a single camelCase formatted string
- The input string is passed to the camelcase function to compute the number of words.
- This function processes the input string s character by character using .chars().
- The .filter() method is used to retain only uppercase characters, which indicate the beginning of a new word.
- The .count() method returns the total number of uppercase letters, representing the number of additional words after the first one.
- Since camelCase always contains at least one lowercase word, the function adds 1 to the count of uppercase letters to account for the first word in the string.
- The result is cast to i32 and written to a file specified by the OUTPUT_PATH environment variable.

## Non divisible subset
- The function reads from standard input.
- The first line contains two space-separated integers, n (the number of elements) and k (the divisor), which are parsed accordingly.
- The second line contains space-separated integers representing the array elements.
- The parsed values are passed to the nonDivisibleSubset function to determine the result.
- This function initialises vector remainder_count of size k to store the frequency of numbers based on their remainder when divided by k.
- Each number in the array s is processed to calculate its remainder, ensuring that negative numbers are handled correctly.
- The frequency of each remainder is updated accordingly.
- A subset can include at most one number that gives a remainder of 0 when divided by k, because including more than one such number would result in a sum divisible by k.
- This is handled by checking remainder_count[0] > 0, and if true, the subset includes one such element.
- The algorithm processes remainders in pairs. For each remainder i, its complement remainder k-i is considered.
- The maximum value between the two remainder groups is added to the result, ensuring that the subset remains valid.
- When i == k-i, only one number from this group can be included, as adding two such numbers would result in a sum divisible by k.
- It function returns the count of numbers that can form the largest non-divisible subset.
- The result is written to a file specified by the OUTPUT_PATH environment variable

## Time in words
- The function reads input from standard input,the first line containing the integer h representing the hour and the second line contains the integer m representing the minutes.
- The parsed values are passed to the timeInWords function to generate the time in words.
- In this function, an array numbers is initialized with string representations of numbers from 0 to 30, including special words for "quarter" and "half".
- This array helps in converting numerical values to their word equivalents efficiently.
- When m = 0, the function returns the hour followed by "o' clock".
    - If m = 1, it returns "one minute past" the given hour.
    - If m = 15, it returns "quarter past" the given hour.
    - If m = 30, it returns "half past" the given hour.
    - If m = 45, it returns "quarter to" the next hour.
    - If m = 59, it returns "one minute to" the next hour.
- Handling general time cases:
    - For values 1 ≤ m ≤ 29, the function constructs the phrase "X minutes past Y" using the numbers array for both minutes and hours.
    - For values 31 ≤ m ≤ 59, the function constructs the phrase "X minutes to Y", where X is 60 - m, and Y is the next hour. 
- Handling invalid input
    - If m does not fall within the expected range, the function returns "Invalid input". 
- The function returns the constructed English phrase representing the given time.
- The result is written to a file specified by the OUTPUT_PATH environment variable.
