# Hints for Add Two Numbers

## Key Question

*   How do you handle the carry-over when the sum of two digits (plus any existing carry) exceeds 9?

## Additional Hints

*   Since the digits are in reverse order, you can process the lists from head to tail, simulating elementary school addition.
*   You'll need a variable to keep track of the `carry` between digit additions.
*   What happens if one list is longer than the other? How do you handle the remaining digits of the longer list?
*   Consider using a dummy head node for the result list to simplify the insertion of the first node.
*   Don't forget the case where a final carry might require adding one last node to the result (e.g., 5 + 5 = 10, resulting list is [0, 1]).
