1. Solve Me First problem - Easy - 0.5 points
   https://www.hackerrank.com/challenges/solve-me-first/problem?isFullScreen=true  
   Function:  
     solveMeFirst computes the sum of two integers and returns the result.  
   
   Main Function:  
     Declares two mutable strings to store user input. Mutability (mut) is used for variables to allow modification during input handling.  
     Reads input lines and handles potential input errors using .ok().expect(...).  
     Trims whitespace and parses the input strings into integers, handling parsing errors similarly.  
     Calls solveMeFirst with the parsed integers and prints the result.  

2. Simple Array Sum problem - Easy - 0.5 points  
   https://www.hackerrank.com/challenges/simple-array-sum/problem?isFullScreen=true  
   simpleArraySum  Function:  
   Takes a slice of integers (&[i32]) as input.  
   Uses .iter() to create an iterator over the array and .sum::<i32>() to compute the sum.  
   Returns the sum as an i32.  
   
   Main Function:  
   Reads input using io::stdin(), where the first line is the array size (unused) and the second line contains space-separated integers.  
   The integers are parsed and stored in a Vec<i32>.  
   Calls simpleArraySum to compute the sum of the array.  
   The result is written to a file using the OUTPUT_PATH environment variable.  
   
3. Compare the Triplets problem - Easy - 0.5 points  
   https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true  
   compareTriplets Function:  
   Takes two input arrays (a and b) representing the ratings of Alice and Bob.  
   Initializes a scores vector to keep track of Alice’s and Bob’s points.  
   Loops over the indices of the arrays (from 0 to 2).  
   Compares the values at each index:  
     If Alice's rating (a[i]) is greater than Bob's (b[i]), Alice gets a point.  
     If Bob's rating (b[i]) is greater than Alice's, Bob gets a point.  
     If their ratings are equal, no one gets a point.  
   Returns the final score as a vector [Alice's score, Bob's score].  

   Main Function:  
   Reads the input from the console using io::stdin(). The first line is Alice's ratings, and the second line is Bob's ratings.  
   Each line of input is split by spaces, converted into integers, and stored in the vectors a and b.  
   The compareTriplets function is called with a and b as arguments, and the result is stored in result.  
   The result is written to a file using the OUTPUT_PATH environment variable.  
   The points are printed in the format [Alice's score, Bob's score].  

4. A Very Big Sum problem - Easy - 0.5 points
   https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=true 
   aVeryBigSum Function:  
  
   Takes a reference to a slice of i64 integers as input (ar).  
   Calculates the sum of all elements in the array using .iter() to create an iterator over the array and .sum() to compute the sum of the elements.  
   It returns the sum as an i64 (64-bit integer), which is capable of handling very large numbers.  

   Main Function:  
   Reads input from the standard input using io::stdin().  
   The integers are parsed from the input string and stored in a Vec<i64>.  
   The aVeryBigSum function is called with the vector ar to calculate the sum.  
   The result is written to a file using the OUTPUT_PATH environment variable.  
   
5. Diagonal Difference problem - Easy - 0.5 points
   https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=true
   diagonalDifference Function:  
   Computes the absolute difference between the sums of the two diagonals of a square matrix.  
    Left to Right Diagonal:  
     The sum of the elements from the top-left to the bottom-right (arr[i][i] for all i).  
    Right to Left Diagonal:  
     The sum of the elements from the top-right to the bottom-left (arr[i][n-i-1] for all i).  
   Returns the absolute difference between them using the .abs() method.  

   Main Function:  
   Reads the input to get the size of the matrix n and then reads the matrix arr row by row.  
   The matrix is stored as a Vec<Vec<i32>> where each element of the outer vector represents a row of the matrix.  
   Calls the diagonalDifference function to calculate the result.  
   The result is written to a file using the OUTPUT_PATH environment variable.  

6. Plus Minus problem - Easy - 0.5 points  
   https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=true  
   plusMinus Function:  
   Accepts a reference to an array (arr) containing integers.  
   Initializes three counters: positive_count, negative_count, and zero_count to track the number of positive, negative, and zero elements in the array.  
   The array is then iterated through, checking the value of each element:  
     If the element is greater than zero, it's a positive number, and positive_count is incremented.  
     If the element is less than zero, it's a negative number, and negative_count is incremented.  
     If the element is equal to zero, zero_count is incremented.  
   After counting the positive, negative, and zero values, the ratios are calculated by dividing each count by the total number of elements in the array.  
   Each ratio is printed with exactly six decimal places using the println! macro with the format "{:.6}".  

   Main Function:  
   It reads the input using stdin.  
   The size of the array is read and then ignored since arr.len() was used directly to determine the length of the array.  
   The array elements are read, split by spaces, and parsed into integers before being stored in a Vec<i32>.  
   The plusMinus function is called to calculate and print the ratios.  
   
7. Staircase problem - Easy - 0.5 points  
   https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true
   staircase Function:  
   Accepts an integer n (size of the staircase).  
   Loops from 1 to n, representing each row of the staircase.  
   In each iteration:  
     Prints (n - i) spaces.  
     Prints i # symbols.  
   After each row, moves to the next line.  

   Main Function:  
   Reads the input size n using stdin.  
   Calls the staircase(n) function to print the staircase of size n.  
   
8. Mini-Max Sum problem - Easy - 0.5 points  
   https://www.hackerrank.com/challenges/mini-max-sum/problem?isFullScreen=true  
   miniMaxSum Function:  
   Sets min to the first element of the array (arr[0]) to track the minimum value.  
   Sets max to the first element of the array (arr[0]) to track the maximum value.  
   Initializes sum to 0 (i64) to store the sum of all elements in the array.  
   Loops through the array to:  
     Add each element num to sum after converting it to i64.  
     If num is smaller than min, update min with num.  
     If num is larger than max, update max with num.  
   Calculates min_sum by subtracting max from sum (sum of the smallest 4 values).  
   Calculates max_sum by subtracting min from sum (sum of the largest 4 values).  
   Print min_sum and max_sum.  

   Main Function:  
   Reads a single line of input and splits it into integers.  
   Calls miniMaxSum with the input array.  
   Prints the result directly from miniMaxSum.  
   
9. Birthday Cake Candles problem - Easy - 0.5 points  
   https://www.hackerrank.com/challenges/birthday-cake-candles/problem?isFullScreen=true  
   birthdayCakeCandles Function:  
   Uses .max() to find the maximum height in the candles array.  
   Filters and counts how many candles match the tallest height using .filter() and .count().  

   Main function:  
   Reads the number of candles (not used) and their heights from the input.  
   Calls birthdayCakeCandles with the candle heights and stores the result.  
   The result is written to a file using the OUTPUT_PATH environment variable.  
    
10. Time Conversion problem - Easy - 0.5 points  
    https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true  
    timeConversion Function:  
    Extracts the last two characters of the string to determine whether it’s AM or PM.  
    Converts hours:  
      If it’s AM and the hour is 12, changes the hour to 00 (midnight).  
      If it’s PM and the hour is not 12, adds 12 to convert to 24-hour format.  
    Formats the hours, minutes, and seconds to ensure a two-digit hour representation (e.g., 07 instead of 7).  
    Returns the formatted 24-hour time string.  

    Main function:  
    Reads the time string from input.  
    Calls the timeConversion function to convert the time to 24-hour format.  
    The result is written to a file using the OUTPUT_PATH environment variable.  
    
11. Forming a Magic Square problem - Medium - 1 point  
    https://www.hackerrank.com/challenges/magic-square-forming/problem?isFullScreen=true  
    formingMagicSquare Function:  
    Has as input a 3x3 matrix (s).  
    Predefine all 8 possible 3x3 magic squares.  
    Cost Calculation:  
      For each magic square, calculates the cost of transforming the given square into it.  
      Adds the absolute difference of corresponding elements in the two squares.  
    Minimum Cost:  
      Tracks and returns the minimum cost.  

    Main Function  
    Reads the 3x3 matrix as input from the user.  
    Passes the matrix to the formingMagicSquare function.  
    Prints the minimum transformation cost.  
    
12. Number Line Jumps problem - Easy - 0.5 points  
    https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem?isFullScreen=true  
    kangaroo Function:  
    Checks if kangaroos can meet:  
      If velocities (v1 and v2) are equal:  
        If starting positions (x1 and x2) are the same, return "YES".  
        Otherwise, return "NO".  
    If velocities differ:  
      Calculates if (x2 - x1) is divisible by (v1 - v2) and if the result is positive (indicating the kangaroos meet after some number of jumps).  
      If true, return "YES". Otherwise, return "NO".
    
    Main Function:  
    Calls kangaroo for each test case (position and velocity) and prints the result.  

13. Between Two Sets problem - Easy - 0.5 points  
    https://www.hackerrank.com/challenges/between-two-sets/problem?isFullScreen=true  
    getTotalX Function:   
    Computes the LCM of all elements in a and the GCD of all elements in b.  
    Checks how many multiples of the LCM are divisors of the GCD.  
    Returns the count of such numbers.  

    Main Function:  
    Reads the two input arrays, a and b.  
    Calls getTotalX to find how many numbers satisfy the condition.  
    Prints the result.  
    
14. Breaking the Records problem - Easy - 0.5 points  
    https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem?isFullScreen=true  
    breakingRecords Function:  
    Takes a slice of integers representing scores.  
    Initializes high_score and low_score with the first element in the array.  
    Initializes counters high_count and low_count to 0.  
    Iterates through the scores starting from the second element:  
    If a score exceeds high_score, it updates high_score and increments high_count.  
    If a score is lower than low_score, it updates low_score and increments low_count.  
    Returns a vector containing the counts of high and low record breaks.  

    Main Function:  
    Reads the input for the number of scores and the list of scores.  
    Calls the breakingRecords function with the scores to get the counts of high and low record breaks.  
    Prints the result as two space-separated integers for the high and low record breaks.  
    
15. Subarray Division problem - Easy - 0.5 points  
    https://www.hackerrank.com/challenges/the-birthday-bar/problem?isFullScreen=true  
    birthday Function:  
    Takes a reference to a vector s representing the chocolate bar, an integer d representing the length of the segment, and an integer m representing the target sum.  
    Iterates through possible segments of length m in the array s.  
    For each segment, it checks if the sum of the segment equals d.  
    If the sum equals d, the count is incremented.  
    Finally, it returns the count of valid segments.  

    Main Function:  
    Defines the chocolate array s, the segment length d, and the target sum m.  
    Calls the birthday() function with the chocolate array and prints the result.  
    
16. Divisible Sum Pairs problem - Easy - 0.5 points  
    https://www.hackerrank.com/challenges/divisible-sum-pairs/problem?isFullScreen=true  
    divisibleSumPairs Function:  
    Receives the length n of the array, the divisor k, and a reference to the array ar.  
    Initializes an array remainder_counts of size k to store the count of each remainder when elements in ar are divided by k.  
    Iterates through each element of ar:  
      Calculates the remainder when the current element is divided by k.  
      Determines the complement remainder that would make the sum divisible by k.  
      Adds the number of previously seen elements that have the complement remainder to the count.  
      Increments the count for the current remainder.  
    Returns the total count of valid pairs.  

    Main Function:  
    Initializes the array ar, the size n, and the divisor k.  
    Calls the divisibleSumPairs function and prints the result.  
    
17. Migratory Birds problem - Easy - 0.5 points  
    https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true  
    migratoryBirds Function:  
    Initializes a HashMap to count the frequency of each bird type.  
    Iterates through the array to populate the map with the bird counts.  
    Iterates through the map to find the bird type with the maximum frequency, breaking ties by choosing the smaller bird type.  
    Returns the bird type with the highest frequency or the smallest one in case of a tie.  

    Main Function:  
    Initializes the input array with bird types.  
    Calls the migratoryBirds function to find the most frequent bird type.  
    Prints the result.


16 x 0.5 (16 Easy problems) + 1 X 1 (Medium problem) = 9
