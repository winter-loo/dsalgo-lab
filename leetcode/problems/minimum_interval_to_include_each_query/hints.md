# Hints for Minimum Interval to Include Each Query

1. Think about how to efficiently find all intervals that contain a given query point.

2. Consider sorting the queries and processing them in order. This allows you to add intervals to a data structure as you encounter them.

3. For each query, you need to find the smallest interval that contains it. A priority queue (min-heap) can help you keep track of the intervals ordered by their size.

4. Remember to remove intervals that are no longer relevant (i.e., intervals that end before the current query point).

5. To optimize the solution, you can sort both the intervals and the queries. As you process each query, you only need to consider intervals that start at or before the query point.

6. Be careful to keep track of the original indices of the queries, as you need to return the answers in the original order.

7. For each query, you need to find intervals where `left <= query <= right`. The size of the interval is `right - left + 1`.

8. Consider what happens when multiple intervals contain a query point. You need to find the one with the smallest size.

9. The time complexity of the solution using a priority queue is O((n + q) log (n + q)), where n is the number of intervals and q is the number of queries.

10. Remember to handle the case when no interval contains a query point. In this case, the answer is -1.
