# Intuition
parcours récursif en léger

# Approach
bubble sort goes brrrr

should have used something clean with recursivity maybe, but didn t feel like it tonight ... while come back on it during the week !

# Complexity
    Time complexity: 138ms (<98.62% of ts users)

    Space complexity:72.89 MB (<24% of ts users)

# Code

```ts
/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function sortList(head: ListNode | null): ListNode | null {
    if (!head) return null;
    let arr = [];
    let node = head;
    while (node) {
        arr.push(node.val);
        node = node.next;
    }
    arr.sort((a, b) => a - b);
    node = head;
    for (let i = 0; i < arr.length; i++) {
        node.val = arr[i];
        node = node.next;
    }
    return head; 
};
```
