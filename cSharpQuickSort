using System;

public class QuickSort
{
    // Main method for demonstration
    public static void Main(string[] args)
    {
        int[] array = {12, 4, 5, 6, 7, 3, 1, 15};
        
        Console.WriteLine("Original Array:");
        PrintArray(array);

        // Applying quick sort
        Sort(array, 0, array.Length - 1);

        Console.WriteLine("\nSorted Array:");
        PrintArray(array);
    }

    // Quick sort method
    public static void Sort(int[] data, int low, int high)
    {
        if (low >= high)
        {
            return;
        }

        // Find the pivot element such that elements smaller than the pivot are on the left,
        // and elements greater than the pivot are on the right
        int pivot = Partition(data, low, high);

        // Recursively sort the sub-arrays on the left and right of the pivot
        Sort(data, low, pivot - 1);
        Sort(data, pivot + 1, high);
    }

    // Partition method to find the correct position of the pivot element
    private static int Partition(int[] data, int low, int high)
    {
        // Choose the last element as the pivot
        int pivot = data[high];
        int i = (low - 1);

        // Traverse the array and move elements smaller than the pivot to the left
        for (int j = low; j <= high - 1; j++)
        {
            if (data[j] <= pivot)
            {
                i++;
                // Swap elements at indices i and j
                Swap(data, i, j);
            }
        }

        // Swap the pivot element with the element at the correct position
        Swap(data, i + 1, high);

        // Return the index of the pivot element
        return i + 1;
    }

    // Utility method to swap elements in the array
    private static void Swap(int[] data, int i, int j)
    {
        int temp = data[i];
        data[i] = data[j];
        data[j] = temp;
    }

    // Utility method to print an array
    private static void PrintArray(int[] array)
    {
        foreach (var value in array)
        {
            Console.Write(value + " ");
        }
        Console.WriteLine();
    }
}
